use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct WeatherAttributes {
    pub temperature_unit: Option<String>,
    pub dew_point: Option<f64>,
    pub pressure: Option<f64>,
}
