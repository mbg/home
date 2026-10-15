use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Attributes {
    pub cloud_coverage: Option<f64>,
    pub dew_point: Option<f64>,
    pub humidity: Option<f64>,
    pub precipitation_unit: Option<String>,
    pub pressure_unit: Option<String>,
    pub pressure: Option<f64>,
    pub temperature_unit: Option<String>,
    pub uv_index: Option<f64>,
    pub visibility_unit: Option<String>,
    pub wind_bearing: Option<f64>,
    pub wind_speed_unit: Option<String>,
    pub wind_speed: Option<f64>,
}
