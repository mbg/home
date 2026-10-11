use serde::{Deserialize, Serialize};

pub mod weather;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Attributes {
    #[serde(flatten)]
    pub weather: weather::Attributes,
}
