use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub unknown: bool,
}
