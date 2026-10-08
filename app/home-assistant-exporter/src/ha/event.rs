use std::{fmt::Display, str::FromStr};

use hass_rs::{EventData, HassEntity, HassEvent, WSEvent};
use tracing::{Level, error, event, info, trace};

use crate::metrics::{self, StateLabels};

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
    Other(String),
}

impl Display for StateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateValue::Unknown => write!(f, "unknown"),
            StateValue::Unavailable => write!(f, "unavailable"),
            StateValue::Numeric(v) => write!(f, "{}", v),
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
        }

        return Ok(StateValue::Other(val));
    }
}

/// Gets the ID of the entity, including the domain.
fn entity_id(event: &HassEvent) -> &Option<String> {
    return &event.data.entity_id;
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

fn update_enum(labels: &mut StateLabels, state_value: String, old_state: Option<String>) {
    // Clear the series based on the old state.
    if let Some(old_state) = old_state {
        clear_enum_for_old_state(&labels, old_state);
    }

    // Set the metric for the current state to 1.
    labels.state = Some(state_value);
    let _ = metrics::STATES.get_or_create(&labels).set(1f64);
}

/// Processes a "state_changed" event.
#[tracing::instrument]
fn state_changed(event: HassEvent) -> Option<()> {
    // Deconstruct the event data.
    let EventData {
        entity_id,
        new_state,
        old_state,
        extra: _extra,
    } = event.data;

    // Check that we have an entity_id. Fail if not.
    let entity_id = match entity_id {
        Some(s) => s,
        None => {
            error!("Event without an entity_id.");
            return None;
        }
    };

    // Extract the domain and name from the entity_id.
    // Fail if the format doesn't match our expectations.
    let (domain, name) = match domain_and_name(&entity_id) {
        Some(r) => r,
        None => {
            error!("Invalid entity_id '{}'.", entity_id);
            return None;
        }
    };

    // Deconstruct the new entity state or fail if there isn't one.
    let HassEntity {
        state, attributes, ..
    } = match new_state {
        Some(s) => s,
        None => {
            error!("No new_state for event.");
            return None;
        }
    };

    // Parse the state.
    let state_value = state.parse::<StateValue>().ok();

    let mut labels =
        metrics::StateLabels::new(entity_id.to_string(), domain.to_string(), name.to_string());

    if let Some(state_value) = state_value {
        if let StateValue::Numeric(val) = state_value {
            metrics::STATES.get_or_create(&labels).set(val);
        } else if state_value == StateValue::Unknown || state_value == StateValue::Unavailable {
            if metrics::STATES.remove(&labels) {
                trace!(
                    "Removed metric for '{}' since it changed to '{:?}'",
                    labels.entity_id, state_value
                );
            }
        } else if domain == "enum" || domain == "event" {
            update_enum(
                &mut labels,
                state_value.to_string(),
                old_state.map(|s| s.state),
            );
        } else {
            info!(
                "Didn't know what to do with an event for '{}' with state '{}'.",
                entity_id, state_value
            );
            return None;
        }
    } else {
        info!(
            "Didn't know what to do with an event for '{}' without state.",
            entity_id
        );
        return None;
    }

    return Some(());
}

/// Processes a web socket event from Home Assistant.
pub async fn process(message: WSEvent) {
    let event = message.event;

    metrics::EVENT_COUNTER
        .get_or_create(&metrics::EventLabels {
            event_type: event.event_type.clone(),
        })
        .inc();

    if event.event_type == "state_changed" {
        state_changed(event);
    } else {
        event!(
            Level::INFO,
            message = "Ignoring unexpected event type.",
            event_type = event.event_type
        )
    }
}
