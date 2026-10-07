use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use prometheus_client::registry::Registry;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::signal::unix::{SignalKind, signal};
use tracing::{error, info};

#[tracing::instrument]
async fn handle_tcp_connection(registry: Arc<Registry>, server: http1::Builder, stream: TcpStream) {
    let mut shutdown_stream = signal(SignalKind::terminate()).unwrap();
    let io = TokioIo::new(stream);
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
            handle_tcp_connection(registry.clone(), server.clone(), stream).await;
        }

        info!("No longer accepting new TCP connections.")
    }));
}
