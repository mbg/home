use hass_rs::HassClient;
use std::{error::Error, process::ExitCode};
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

mod config;
mod ha;
mod metrics;
mod server;

/// The name of the environment variable that we expect the tracing filter
/// configuration in.
static ENV_VAR_LOG_FILTER: &str = "HOME_LOG_FILTER";

/// Logs `message` as a rudimentary JSON object to stderr. Used before
/// the json_subscriber is initialised to maintain a consistent output format.
fn eprintln_json(message: &str, err: Box<dyn Error + Send + Sync + 'static>) {
    eprintln!("{{ \"message\": \"{}: {}\" }}", message, err);
}

/// Initialises the tracing subscriber based on the available configuration.
fn init_tracing_subscriber()
-> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Try to construct a filter based on the `HOME_LOG_FILTER` environment variable.
    let filter = match EnvFilter::try_from_env(ENV_VAR_LOG_FILTER) {
        Ok(f) => f,
        Err(err) => {
            eprintln_json(
                format!("Failed to parse '{}'", ENV_VAR_LOG_FILTER).as_str(),
                Box::new(err),
            );
            EnvFilter::default()
        }
    }
    // Default to Level::INFO.
    .add_directive(LevelFilter::INFO.into());

    // Construct a global tracing subscriber based on the filter.
    let subscriber = json_subscriber::fmt();
    return subscriber.with_env_filter(filter).try_init();
}

#[tokio::main]
async fn main() -> ExitCode {
    // Initialise the tracing subscriber so that the configuration for it
    // will apply to all log messages.
    if let Err(err) = init_tracing_subscriber() {
        // If this has failed, log the error to stderr and exit.
        eprintln_json("Failed to initialise tracing subscriber", err);
        return ExitCode::FAILURE;
    }

    // Log something to show that we are alive.
    info!("Starting home-assistant-exporter...");

    // Obtain the service configuration.
    let config = config::load();

    // Initialise the metrics registry.
    let registry = metrics::create();

    // Start the HTTP server to serve the metrics.
    let metrics_addr =
        std::net::SocketAddr::new(config.server.address, config.server.port);
    let metrics_listener;

    match server::start(metrics_addr, registry.clone()).await {
        None => return ExitCode::FAILURE,
        Some(listener) => metrics_listener = listener,
    }

    // Initialise the Home Assistant web socket client.
    let mut ha_client: HassClient;
    match ha::connect(&config.ha).await {
        None => return ExitCode::FAILURE,
        Some(client) => ha_client = client,
    }

    info!("Successfully connected to Home Assistant.");

    // Subscribe to `state_changed` events from Home Assistant.
    let mut event_receiver;
    match ha_client.subscribe_event("state_changed").await {
        Err(err) => {
            error!("Failed to subscribe to `state_changed` events: {}", err);
            return ExitCode::FAILURE;
        }
        Ok(receiver) => event_receiver = receiver,
    };

    // Spawn a task to handle events we receive from Home Assistant.
    let event_listener = tokio::spawn(async move {
        while let Some(message) = event_receiver.recv().await {
            ha::event::process(message).await;
        }
        info!("Connection to Home Assistant has been closed.");
    });

    info!(
        "Successfully subscribed to `state_changed` events and started listening for them."
    );

    // Wait for any one of the tasks to complete.
    tokio::select! {
        event_result = event_listener => {
            if let Err(error) = event_result {
                error!("Error while waiting for the event listener: {}", error);
                return ExitCode::FAILURE;
            }
        },
        metrics_result = metrics_listener => {
            if let Err(error) = metrics_result {
                error!("Error while waiting for the metrics listener: {}", error);
                return ExitCode::FAILURE;
            }
        }
    }

    // If we have reached this point, we are exiting normally.
    return ExitCode::SUCCESS;
}
