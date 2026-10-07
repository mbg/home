use std::str::FromStr;

use hass_rs::{HassEvent, WSEvent};
use tracing::{Level, event, info, trace};

use crate::metrics::{self};

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

        return Err(());
    }
}

/// Gets the ID of the entity, including the domain.
fn entity_id(event: &HassEvent) -> &Option<String> {
    return &event.data.entity_id;
}

/// Processes a "state_changed" event.
#[tracing::instrument]
fn state_changed(event: HassEvent) -> Option<()> {
    let entity_id = entity_id(&event).as_ref()?;
    let state_value = event
        .data
        .new_state
        .as_ref()
        .and_then(|s| s.state.parse::<StateValue>().ok());

    let labels = metrics::StateLabels {
        entity_id: entity_id.to_string(),
    };

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
        }
    } else {
        info!(
            "Didn't know what to do with an event for '{}' with state '{:?}'.",
            entity_id, state_value
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
