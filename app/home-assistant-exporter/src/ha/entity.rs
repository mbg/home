use serde::{Deserialize, Serialize};

pub mod device_tracker;
pub mod weather;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Attributes {
    #[serde(flatten)]
    pub device_tracker: device_tracker::Attributes,

    #[serde(flatten)]
    pub weather: weather::Attributes,
}
