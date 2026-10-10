use std::env::var;

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

pub fn get_server_config() -> ServerConfig {
    let address = var("HOME_SERVER_ADDRESS")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(std::net::IpAddr::V4(DEFAULT_METRIC_ADDRESS));
    let port = var("HOME_SERVER_PORT")
        .ok()
        .and_then(|val| val.parse::<u16>().ok())
        .unwrap_or(DEFAULT_METRIC_PORT);

    return ServerConfig { address, port };
}

pub fn get_home_assistant_config() -> HomeAssistantConfig {
    let token: String = var("HOME_HA_TOKEN").expect(
        "A token must be configured in the HOME_HA_TOKEN environment variable.",
    );
    let server: String = var("HOME_HA_SERVER")
        .expect("The hostname of the Home Assistant server must be configured in the HOME_HA_SERVER environment variable.");

    return HomeAssistantConfig { token, server };
}

/// Loads the configuration.
pub fn load() -> HomeAssistantExporterConfig {
    let server = get_server_config();
    let ha = get_home_assistant_config();
    return HomeAssistantExporterConfig { server, ha };
}
