use hass_rs::HassClient;
use std::process::ExitCode;
use tracing::{error, info};

mod config;
mod ha;
mod metrics;

#[tokio::main]
async fn main() -> ExitCode {
    // Install global collector configured based on the RUST_LOG env var.
    json_subscriber::fmt::init();

    // Log something to show that we are alive.
    info!("Starting home-assistant-exporter...");

    // Obtain the service configuration.
    let config = config::load();

    // Initialise the metrics registry.
    let registry = metrics::create();

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

    info!("Successfully subscribed to `state_changed` events and started listening for them.");

    // Wait for the task to complete.
    tokio::select! {
        event_result = event_listener => {
            if let Err(error) = event_result {
                error!("Error while waiting for the event listener: {}", error);
                return ExitCode::FAILURE;
            }
        },
    }

    // If we have reached this point, we are exiting normally.
    return ExitCode::SUCCESS;
}
