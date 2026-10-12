use std::{
    env::{VarError, var},
    fmt::Display,
};

use clap::Parser;
use serde::Deserialize;

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

/// The default path of the configuration file.
static DEFAULT_CONFIG_FILE: &str = "config/home-assistant.yml";

pub static DEFAULT_METRIC_PORT: u16 = 8001;

pub static DEFAULT_METRIC_ADDRESS: std::net::Ipv4Addr =
    std::net::Ipv4Addr::from_octets([0, 0, 0, 0]);

/// The default hostname for the HA server.
static DEFAULT_HA_SERVER: &str = "homeassistant.local";

/// The default prefix for prometheus metrics.
pub static DEFAULT_METRIC_PREFIX: &str = "ha";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CmdArgs {
    /// Path to the configuration file to use.
    config_file: Option<String>,
}

fn default_metric_address() -> std::net::IpAddr {
    return std::net::IpAddr::V4(DEFAULT_METRIC_ADDRESS);
}

fn default_metric_port() -> u16 {
    return DEFAULT_METRIC_PORT;
}

fn default_metric_prefix() -> String {
    return DEFAULT_METRIC_PREFIX.to_string();
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    /// The address that the metrics server should listen at.
    #[serde(default = "default_metric_address")]
    pub address: std::net::IpAddr,

    /// The port that the metrics server should listen at.
    #[serde(default = "default_metric_port")]
    pub port: u16,

    /// The prefix to use for metric names.
    #[serde(default = "default_metric_prefix")]
    pub prefix: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: default_metric_address(),
            port: default_metric_port(),
            prefix: default_metric_prefix(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct HomeAssistantConfig {
    /// The long-lived token to authenticate to HA with.
    #[serde(skip)]
    pub token: String,

    /// The hostname of the HA server.
    #[serde(default)]
    pub server: String,
}

impl Default for HomeAssistantConfig {
    fn default() -> Self {
        Self {
            token: Default::default(),
            server: DEFAULT_HA_SERVER.to_string(),
        }
    }
}

pub mod debug;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HomeAssistantExporterConfig {
    #[serde(default)]
    pub debug: debug::Config,

    /// The configuration for the metrics server.
    #[serde(default)]
    pub server: ServerConfig,

    /// The configuration needed to connect to Home Assistant.
    #[serde(default)]
    pub home_assistant: HomeAssistantConfig,
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

impl Display for EnvVarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvVarError::NoValue { message, var } => {
                write!(f, "Environment variable '{}': {}", var, message)
            }
            EnvVarError::WithValue {
                message,
                var,
                value,
            } => write!(
                f,
                "Environment variable '{}' with value '{}': {}",
                var, value, message
            ),
        }
    }
}

pub enum ConfigError {
    IOError(std::io::Error),
    YAMLError(serde_yaml::Error),
    EnvVarError(EnvVarError),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IOError(error) => {
                write!(f, "Unable to read configuration file: {}", error)
            }
            ConfigError::YAMLError(error) => {
                write!(f, "Unable to parse configuration file: {}", error)
            }
            ConfigError::EnvVarError(env_var_error) => env_var_error.fmt(f),
        }
    }
}

/// Get and parse an optional environment variable. Returns `None` if the
/// environment variable is not set or can't be parsed.
fn optional_env_var<T>(key: &str) -> Option<T>
where
    T: std::str::FromStr,
{
    return var(key).ok().and_then(|s| s.parse().ok());
}

/// Get a required environment variable. Returns the value as `String` or
/// a description of the error if the variable is not set.
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

/// Apply environment variable overrides to `config`.
pub fn server_config_from_env(config: &mut ServerConfig) {
    if let Some(address) = optional_env_var(ENV_VAR_SERVER_ADDRESS) {
        config.address = address;
    }
    if let Some(port) = optional_env_var(ENV_VAR_SERVER_PORT) {
        config.port = port;
    }
}

/// Apply environment variable overrides to `config`.
pub fn home_assistant_config_from_env(
    config: &mut HomeAssistantConfig,
) -> Result<(), EnvVarError> {
    let token: String =
        required_env_var(ENV_VAR_HA_TOKEN, "A token must be configured")?;
    config.token = token;

    if let Some(server) = optional_env_var(ENV_VAR_HA_SERVER) {
        config.server = server;
    }

    return Ok(());
}

/// Loads the configuration file specified in `cmd_args` or the default location.
/// Then applies any environment variables that hold configuration overrides.
pub fn load(
    cmd_args: CmdArgs,
) -> Result<HomeAssistantExporterConfig, ConfigError> {
    // Open the configuration file. This must exist, but may be empty.
    let config_file = std::fs::File::open(
        cmd_args
            .config_file
            .unwrap_or(DEFAULT_CONFIG_FILE.to_string()),
    )
    .map_err(ConfigError::IOError)?;

    // Parse the configuration file.
    let mut config: HomeAssistantExporterConfig =
        serde_yaml::from_reader(config_file).map_err(ConfigError::YAMLError)?;

    // Inspect environment variables and use them to override the configuration.
    server_config_from_env(&mut config.server);
    home_assistant_config_from_env(&mut config.home_assistant)
        .map_err(ConfigError::EnvVarError)?;

    return Ok(config);
}
