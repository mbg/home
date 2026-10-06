use std::process::ExitCode;
use tracing::info;

mod config;

#[tokio::main]
async fn main() -> ExitCode {
    // Install global collector configured based on the RUST_LOG env var.
    json_subscriber::fmt::init();

    // Log something to show that we are alive.
    info!("Starting home-assistant-exporter...");

    // Obtain the service configuration.
    config::load();

    // If we have reached this point, we are exiting normally.
    return ExitCode::SUCCESS;
}
