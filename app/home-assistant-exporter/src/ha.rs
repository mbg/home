use hass_rs::client::HassClient;
use tracing::{error, info};

use crate::config::HomeAssistantConfig;

pub mod event;

/// Attempts to initialise a Home Assistant web socket client for `url`.
/// Will try to authenticate it using `token` if the connection is successful.
/// Returns the authenticated client or nothing.
async fn attempt_connection(url: &str, token: &String) -> Option<HassClient> {
    info!("Connecting to {}", url);

    let client_result = HassClient::new(url).await;

    match client_result {
        Err(hass_err) => {
            error!("Failed to connect to {}: {}", url, hass_err);
            return None;
        }
        Ok(client) => {
            return attempt_auth(client, token).await;
        }
    }
}

/// Attempts to authenticate `client` using `token`.
/// Logs if this is unsuccessful.
/// Returns the authenticated client or nothing.
async fn attempt_auth(mut client: HassClient, token: &String) -> Option<HassClient> {
    let auth_result = client.auth_with_longlivedtoken(token).await;

    match auth_result {
        Err(hass_err) => {
            error!("Failed to authenticate: {}", hass_err);
            return None;
        }
        Ok(()) => return Some(client),
    }
}

pub async fn connect(config: &HomeAssistantConfig) -> Option<HassClient> {
    let server = &config.server;
    let url: String = format!("ws://{server}/api/websocket");

    return attempt_connection(&url, &config.token).await;
}
