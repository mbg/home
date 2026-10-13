use std::{collections::HashMap, fmt::Display, str::FromStr};

use chrono::Utc;
use hass_rs::{EventData, HassEntity, HassEvent, WSEvent};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use tracing::{Level, error, event, info, trace, warn};

use crate::{
    config,
    ha::entity::{self},
    metrics::{self, StateLabels},
};

pub static ENUM_DEVICE_CLASS: &str = "enum";

/// Determines if `value` is `unknown`.
fn is_unknown(value: &String) -> bool {
    return value == "unknown";
}

/// Determines if `value` is `unavailable`.
fn is_unavailable(value: &String) -> bool {
    return value == "unavailable";
}

/// Tries to parse `value` as a boolean value.
fn value_as_bool(value: &String) -> Option<f64> {
    let value_lc = value.to_lowercase();
    if value_lc == "on" {
        return Some(1f64);
    } else if value_lc == "off" {
        return Some(0f64);
    }

    return None;
}

/// Tries to parse `value` into an f64.
fn value_as_f64(value: &String) -> Option<f64> {
    if let Some(val) = value.trim().parse::<f64>().ok() {
        return Some(val);
    }

    return value_as_bool(value);
}

#[derive(Debug, PartialEq)]
pub enum StateValue {
    Unknown,
    Unavailable,
    Numeric(f64),
    DateTime(chrono::DateTime<Utc>),
    Other(String),
}

impl Display for StateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateValue::Unknown => write!(f, "unknown"),
            StateValue::Unavailable => write!(f, "unavailable"),
            StateValue::Numeric(v) => write!(f, "{}", v),
            StateValue::DateTime(dt) => write!(f, "{}", dt),
            StateValue::Other(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for StateValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.trim().to_lowercase();

        if is_unknown(&val) {
            return Ok(StateValue::Unknown);
        } else if is_unavailable(&val) {
            return Ok(StateValue::Unavailable);
        } else if let Some(val) = value_as_f64(&val) {
            return Ok(StateValue::Numeric(val));
        } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
            return Ok(StateValue::DateTime(dt.to_utc()));
        }

        return Ok(StateValue::Other(val));
    }
}

/// De-serialises `val` as `T` or returns the default value for `T`.
fn from_value_or_default<T>(val: Value) -> T
where
    T: DeserializeOwned + Default,
{
    return serde_json::from_value::<T>(val).unwrap_or_default();
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CommonAttributes {
    pub friendly_name: Option<String>,
    pub device_class: Option<String>,
    pub state_class: Option<String>,
    pub unit_of_measurement: Option<String>,
    pub options: Option<Vec<String>>,
    pub attribution: Option<String>,

    #[serde(flatten)]
    pub event_attrs: EventAttributes,

    #[serde(flatten)]
    pub climate_attrs: ClimateAttributes,

    #[serde(flatten)]
    pub group_attrs: GroupAttributes,

    #[serde(flatten)]
    pub camera_attrs: CameraAttributes,

    #[serde(flatten)]
    pub sun_attrs: SunAttributes,

    #[serde(flatten)]
    pub entity_attrs: entity::Attributes,

    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

/// Extracts common attributes from `attrs` and applies them to `labels`.
fn apply_attr_labels(
    mut labels: StateLabels,
    attrs: &CommonAttributes,
) -> StateLabels {
    labels.friendly_name = attrs.friendly_name.clone();
    labels.device_class = attrs.device_class.clone();
    labels.state_class = attrs.state_class.clone();
    labels.unit_of_measurement = attrs.unit_of_measurement.clone();
    return labels;
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct EventAttributes {
    pub event_type: Option<String>,
    pub event_types: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ClimateAttributes {
    pub hvac_action: Option<String>,
    pub hvac_modes: Option<Vec<String>>,
    pub current_temperature: Option<f64>,
    pub max_temp: Option<f64>,
    pub min_temp: Option<f64>,
    pub supported_features: Option<f64>,
    pub target_temp_step: Option<f64>,
    pub temperature: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GroupAttributes {
    pub entity_id: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CameraAttributes {
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub bitrate: Option<u32>,
    pub channel_id: Option<u32>,
    pub fps: Option<u32>,
    pub motion_detection: Option<bool>,
    pub entity_picture: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SunAttributes {
    pub azimuth: Option<f64>,
    pub elevation: Option<f64>,

    pub rising: Option<bool>,

    pub next_setting: Option<String>,
    pub next_rising: Option<String>,
    pub next_dusk: Option<String>,
    pub next_midnight: Option<String>,
    pub next_noon: Option<String>,
    pub next_dawn: Option<String>,
}

/// Gets the HA domain and name from an entity_id.
pub fn domain_and_name(entity_id: &String) -> Option<(&str, &str)> {
    let parts: Vec<&str> = entity_id.split('.').collect();

    if parts.len() == 2 {
        return Some((parts[0], parts[1]));
    }
    return None;
}

fn clear_enum_for_old_state(labels: &StateLabels, old_state: String) {
    // Reset the previous state metric for this enum-like entity
    let mut prev_labels = labels.clone();
    prev_labels.state = Some(old_state);

    metrics::STATES.remove(&prev_labels);
}

fn update_enum(
    labels: &mut StateLabels,
    state_value: String,
    old_state: Option<String>,
    possible_values: Option<Vec<String>>,
) {
    // Clear the series based on the old state.
    if let Some(old_state) = old_state {
        event!(
            Level::DEBUG,
            event_type = "state_change",
            entity_id = labels.entity_id,
            from = old_state,
            to = state_value
        );
        clear_enum_for_old_state(&labels, old_state);
    } else {
        event!(
            Level::DEBUG,
            event_type = "state_change",
            entity_id = labels.entity_id,
            to = state_value
        );
    }

    // Clear the series based on possible values.
    if let Some(possible_values) = possible_values {
        for possible_value in possible_values {
            clear_enum_for_old_state(labels, possible_value);
        }
    }

    // Set the metric for the current state to 1.
    labels.state = Some(state_value);
    let _ = metrics::STATES.get_or_create(&labels).set(1f64);
}

fn update_sub_metric(
    mut labels: StateLabels,
    metric: &str,
    val: Option<f64>,
    unit: Option<String>,
) {
    if let Some(val) = val {
        labels.metric = Some(metric.to_string());
        labels.unit_of_measurement = unit;
        metrics::STATES.get_or_create(&labels).set(val);
    }
}

/// Adds or updates the metric(s) for `entity`.
pub fn add_or_update_entity(
    config: &config::HomeAssistantExporterConfig,
    entity: HassEntity,
    old_state: Option<HassEntity>,
) -> Option<()> {
    let HassEntity {
        entity_id,
        state,
        attributes,
        ..
    } = entity;

    // Extract the domain and name from the entity_id.
    // Fail if the format doesn't match our expectations.
    let (domain, name) = match domain_and_name(&entity_id) {
        Some(r) => r,
        None => {
            error!("Invalid entity_id '{}'.", entity_id);
            return None;
        }
    };

    // Parse the state.
    let state_value = state.parse::<StateValue>().ok();

    let (old_attrs, old_state_value) =
        old_state.map_or((None, None), |s| (Some(s.attributes), Some(s.state)));

    let attributes = from_value_or_default::<CommonAttributes>(attributes);
    let mut labels = apply_attr_labels(
        metrics::StateLabels::new(
            entity_id.to_string(),
            domain.to_string(),
            name.to_string(),
        ),
        &attributes,
    );

    // Report unrecognised attributes to the log so that they aren't
    // just silently dropped.
    for (key, val) in &attributes.unknown {
        for ignore in &config.home_assistant.ignore.attributes {
            if ignore.applies_to_attr(domain, key) {
                continue;
            }
        }
        warn!(
            "Unknown attribute '{}' for '{}': {}",
            key,
            entity_id,
            serde_json::to_string(&val).unwrap_or_default()
        );
    }

    if let Some(state_value) = state_value {
        if let StateValue::Numeric(val) = state_value {
            metrics::STATES.get_or_create(&labels).set(val);
        } else if state_value == StateValue::Unknown
            || state_value == StateValue::Unavailable
        {
            if metrics::STATES.remove(&labels) {
                trace!(
                    "Removed metric for '{}' since it changed to '{:?}'",
                    labels.entity_id, state_value
                );
            }
        } else if attributes.device_class
            == Some(String::from(ENUM_DEVICE_CLASS))
        {
            let possible_values = attributes.options.unwrap_or_default();
            update_enum(
                &mut labels,
                state_value.to_string(),
                old_state_value,
                Some(possible_values),
            );
        } else if attributes.device_class == Some(String::from("timestamp")) {
            info!(
                "Entity '{}' (device class 'timestamp') was updated at '{}'.",
                entity_id, state_value
            );
        } else if domain == "event" {
            let old_attrs = old_attrs
                .map_or(EventAttributes::default(), from_value_or_default);

            let event_attrs = attributes.event_attrs;
            if let Some(event_type) = &event_attrs.event_type {
                event!(
                    Level::INFO,
                    entity_id,
                    event_type = "event_triggered",
                    name = event_type,
                );

                update_enum(
                    &mut labels,
                    event_type.to_string(),
                    old_attrs.event_type,
                    old_attrs.event_types,
                );
            } else {
                warn!("No 'event_type' for '{}' state change.", entity_id);
            }
        } else if domain == "climate" {
            if let Some(current_temperature) =
                attributes.climate_attrs.current_temperature
            {
                event!(
                    Level::INFO,
                    entity_id,
                    event_type = "climate_change",
                    temperature = attributes.climate_attrs.current_temperature
                );

                labels.state = Some(state_value.to_string());
                metrics::STATES
                    .get_or_create(&labels)
                    .set(current_temperature);
            } else {
                warn!("No 'current_temperature' for '{}'.", entity_id);
            }
        } else if domain == "camera" {
            update_enum(
                &mut labels,
                state_value.to_string(),
                old_state_value,
                None,
            );
        } else if domain == "device_tracker" {
            update_enum(
                &mut labels,
                state_value.to_string(),
                old_state_value,
                None,
            );
        } else if domain == "sun" {
            update_sub_metric(
                labels.clone(),
                "azimuth",
                attributes.sun_attrs.azimuth,
                Some("°".to_string()),
            );

            update_sub_metric(
                labels.clone(),
                "elevation",
                attributes.sun_attrs.elevation,
                Some("°".to_string()),
            );

            update_enum(
                &mut labels,
                state_value.to_string(),
                old_state_value,
                None,
            );
        } else if domain == "weather" {
            update_sub_metric(
                labels.clone(),
                "cloud_coverage",
                attributes.entity_attrs.weather.cloud_coverage,
                Some("%".to_string()),
            );

            update_sub_metric(
                labels.clone(),
                "dew_point",
                attributes.entity_attrs.weather.dew_point,
                attributes.entity_attrs.weather.temperature_unit,
            );

            update_sub_metric(
                labels.clone(),
                "humidity",
                attributes.entity_attrs.weather.humidity,
                Some("%".to_string()),
            );

            update_sub_metric(
                labels.clone(),
                "pressure",
                attributes.entity_attrs.weather.pressure,
                attributes.entity_attrs.weather.pressure_unit,
            );

            update_sub_metric(
                labels.clone(),
                "uv_index",
                attributes.entity_attrs.weather.uv_index,
                None,
            );

            update_sub_metric(
                labels.clone(),
                "wind_bearing",
                attributes.entity_attrs.weather.wind_bearing,
                Some("°".to_string()),
            );

            update_sub_metric(
                labels.clone(),
                "wind_speed",
                attributes.entity_attrs.weather.wind_speed,
                attributes.entity_attrs.weather.wind_speed_unit,
            );

            update_enum(
                &mut labels,
                state_value.to_string(),
                old_state_value,
                None,
            );
        } else {
            info!(
                "Didn't know what to do with an event for '{}' with state '{}': {}",
                entity_id,
                state_value,
                serde_json::to_string(&attributes).unwrap_or_default()
            );

            if config.debug.unknown {
                metrics::UNSAVED_ENTITIES.get_or_create(&labels).inc();
            }

            return None;
        }
    } else {
        info!(
            "Didn't know what to do with an event for '{}' without state.",
            entity_id
        );

        if config.debug.unknown {
            metrics::UNSAVED_ENTITIES.get_or_create(&labels).inc();
        }

        return None;
    }

    return Some(());
}

/// Processes a "state_changed" event.
// #[tracing::instrument]
fn state_changed(
    config: &config::HomeAssistantExporterConfig,
    event: HassEvent,
) -> Option<()> {
    // Deconstruct the event data.
    let EventData {
        entity_id: _,
        new_state,
        old_state,
        extra: _extra,
    } = event.data;

    // Deconstruct the new entity state or fail if there isn't one.
    let entity = match new_state {
        Some(s) => s,
        None => {
            error!("No new_state for event.");
            return None;
        }
    };

    return add_or_update_entity(config, entity, old_state);
}

/// Processes a web socket event from Home Assistant.
pub async fn process(
    config: &config::HomeAssistantExporterConfig,
    message: WSEvent,
) {
    let event = message.event;

    metrics::EVENT_COUNTER
        .get_or_create(&metrics::EventLabels {
            event_type: event.event_type.clone(),
        })
        .inc();

    if event.event_type == "state_changed" {
        state_changed(config, event);
    } else {
        event!(
            Level::INFO,
            message = "Ignoring unexpected event type.",
            event_type = event.event_type
        )
    }
}
