use hass_rs::HassClient;
use std::process::ExitCode;
use tracing::info;

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

    // If we have reached this point, we are exiting normally.
    return ExitCode::SUCCESS;
}
