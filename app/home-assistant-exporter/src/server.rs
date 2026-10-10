use futures::future::BoxFuture;
use http_body_util::{BodyExt, Full, combinators};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::pin;
use tokio::signal::unix::{SignalKind, signal};
use tracing::{error, info};

static OPENMETRICS_TEXT: &str =
    "application/openmetrics-text; version=1.0.0; charset=utf-8";

/// Represents boxed HTTP bodies for responses.
type BoxBody = combinators::BoxBody<Bytes, hyper::Error>;

/// Construct a response body from `body`.
pub fn make_body(body: Bytes) -> BoxBody {
    Full::new(body).map_err(|never| match never {}).boxed()
}

/// Constructs a request handler which serves metrics from `registry` to the client.
pub fn metrics_handler(
    registry: Arc<Registry>,
) -> impl Fn(Request<Incoming>) -> BoxFuture<'static, io::Result<Response<BoxBody>>>
{
    move |_request: Request<Incoming>| {
        let registry = registry.clone();

        Box::pin(async move {
            let mut buf = String::new();
            encode(&mut buf, &registry)
                .map_err(std::io::Error::other)
                .map(|_| {
                    let body = make_body(Bytes::from(buf));
                    Response::builder()
                        .header(hyper::header::CONTENT_TYPE, OPENMETRICS_TEXT)
                        .body(body)
                        .unwrap()
                })
        })
    }
}

#[tracing::instrument]
async fn handle_tcp_connection(
    registry: Arc<Registry>,
    server: http1::Builder,
    stream: TcpStream,
) {
    let mut shutdown_stream = signal(SignalKind::terminate()).unwrap();
    let io = TokioIo::new(stream);
    let service = service_fn(metrics_handler(registry));

    tokio::task::spawn(async move {
        let connection = server.serve_connection(io, service);
        pin!(connection);

        // Wait until we have either finished handling the `connection`
        // or this process has received the shutdown signal.
        tokio::select! {
            _ = connection.as_mut() => {}
            _ = shutdown_stream.recv() => {
                // Try to close the connection gracefully.
                connection.as_mut().graceful_shutdown();
            }
        }
    });
}

/// Starts a HTTP server listening at `metrics_addr` which serves metrics from the `registry`.
pub async fn start(
    metrics_addr: SocketAddr,
    registry: Arc<Registry>,
) -> Option<tokio::task::JoinHandle<()>> {
    info!("Starting metrics server on {metrics_addr}");

    // Try to initialise a TCP listener.
    let tcp_listener;
    match TcpListener::bind(metrics_addr).await {
        Err(error) => {
            error!("Failed to create TCP listener: {}", error);
            return None;
        }
        Ok(listener) => tcp_listener = listener,
    }

    let server = http1::Builder::new();

    return Some(tokio::spawn(async move {
        while let Ok((stream, _)) = tcp_listener.accept().await {
            handle_tcp_connection(registry.clone(), server.clone(), stream)
                .await;
        }

        info!("No longer accepting new TCP connections.")
    }));
}
