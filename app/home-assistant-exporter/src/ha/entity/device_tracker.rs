use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Attributes {
    pub host_name: Option<String>,
    pub is_guest: Option<bool>,
    pub authorized: Option<bool>,
    pub tracking_type: Option<String>,
    pub in_zones: Option<Vec<String>>,
    pub qos_policy_applied: Option<bool>,
    pub source_type: Option<String>,
    pub oui: Option<String>,
    pub mac: Option<String>,
}
