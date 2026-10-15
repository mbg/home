use backoff::future::retry;
use backoff::{Error::Permanent, ExponentialBackoff};
use hass_rs::{HassError, client::HassClient};
use tracing::{error, info};

use crate::config::HomeAssistantConfig;

pub mod entity;
pub mod event;

fn hass_error_to_backoff(err: HassError) -> backoff::Error<String> {
    match err {
        HassError::AuthenticationFailed(s) => return Permanent(s),
        _ => return backoff::Error::transient(err.to_string()),
    }
}

/// Attempts to initialise a Home Assistant web socket client for `url`.
/// Will try to authenticate it using `token` if the connection is successful.
/// Returns the authenticated client or nothing.
async fn attempt_connection(
    url: &str,
    token: &String,
) -> Result<HassClient, backoff::Error<String>> {
    info!("Connecting to {}", url);

    let client_result = HassClient::new(url).await;

    match client_result {
        Err(hass_err) => {
            error!("Failed to connect to {}: {}", url, hass_err);
            return Err(hass_error_to_backoff(hass_err));
        }
        Ok(client) => {
            return attempt_auth(client, token).await;
        }
    }
}

/// Attempts to authenticate `client` using `token`.
/// Logs if this is unsuccessful.
/// Returns the authenticated client or nothing.
async fn attempt_auth(
    mut client: HassClient,
    token: &String,
) -> Result<HassClient, backoff::Error<String>> {
    let auth_result = client.auth_with_longlivedtoken(token).await;

    match auth_result {
        Err(hass_err) => {
            error!("Failed to authenticate: {}", hass_err);
            return Err(hass_error_to_backoff(hass_err));
        }
        Ok(()) => return Ok(client),
    }
}

pub async fn connect(config: &HomeAssistantConfig) -> Option<HassClient> {
    let server = &config.server;
    let url: String = format!("ws://{server}/api/websocket");

    return retry(ExponentialBackoff::default(), async || {
        return attempt_connection(&url, &config.token).await;
    })
    .await
    .ok();
}
