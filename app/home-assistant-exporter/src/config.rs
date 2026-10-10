use std::env::{VarError, var};

/// The name of the environment variable that we expect the address the
/// metrics server should listen on in.
static ENV_VAR_SERVER_ADDRESS: &str = "HOME_SERVER_ADDRESS";

/// The name of the environment variable that we expect the port the
/// metrics server should listen on in.
static ENV_VAR_SERVER_PORT: &str = "HOME_SERVER_PORT";

/// The name of the environment variable that we expect the hostname
/// of the Home Assistant server in.
static ENV_VAR_HA_SERVER: &str = "HOME_HA_SERVER";

/// The name of the environment variable that we expect the API token
/// for the Home Assistant instance in.
static ENV_VAR_HA_TOKEN: &str = "HOME_HA_TOKEN";

pub static DEFAULT_METRIC_PORT: u16 = 8001;

pub static DEFAULT_METRIC_ADDRESS: std::net::Ipv4Addr =
    std::net::Ipv4Addr::from_octets([0, 0, 0, 0]);

/// The default prefix for prometheus metrics.
pub static DEFAULT_METRIC_PREFIX: &str = "ha";

pub struct ServerConfig {
    pub address: std::net::IpAddr,
    pub port: u16,
}

pub struct HomeAssistantConfig {
    pub token: String,
    pub server: String,
}

pub struct HomeAssistantExporterConfig {
    pub server: ServerConfig,
    pub ha: HomeAssistantConfig,
}

pub enum EnvVarError {
    NoValue {
        message: String,
        var: String,
    },
    WithValue {
        message: String,
        var: String,
        value: String,
    },
}

pub fn required_env_var(
    name: &str,
    missing: &str,
) -> Result<String, EnvVarError> {
    return var(name).map_err(|err| EnvVarError::NoValue {
        message: match err {
            VarError::NotPresent => format!(
                "{}, but the environment variable was not found.",
                missing,
            ),
            VarError::NotUnicode(os_string) => format!(
                "{}, but the environment variable contained invalid unicode: {:?}",
                missing,
                os_string
            )
        },
        var: name.to_string(),
    });
}

pub fn get_server_config() -> ServerConfig {
    let address = var(ENV_VAR_SERVER_ADDRESS)
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(std::net::IpAddr::V4(DEFAULT_METRIC_ADDRESS));
    let port = var(ENV_VAR_SERVER_PORT)
        .ok()
        .and_then(|val| val.parse::<u16>().ok())
        .unwrap_or(DEFAULT_METRIC_PORT);

    return ServerConfig { address, port };
}

pub fn get_home_assistant_config() -> Result<HomeAssistantConfig, EnvVarError> {
    let token: String =
        required_env_var(ENV_VAR_HA_TOKEN, "A token must be configured")?;

    let server: String = required_env_var(
        ENV_VAR_HA_SERVER,
        "The hostname of the Home Assistant server must be configured",
    )?;

    return Ok(HomeAssistantConfig { token, server });
}

/// Loads the configuration.
pub fn load() -> Result<HomeAssistantExporterConfig, EnvVarError> {
    let server = get_server_config();
    let ha = get_home_assistant_config()?;
    return Ok(HomeAssistantExporterConfig { server, ha });
}
