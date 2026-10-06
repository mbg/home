use hass_rs::{HassEvent, WSEvent};
use tracing::info;

use crate::metrics::{self};

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

/// Gets the ID of the entity, including the domain.
fn entity_id(event: &HassEvent) -> &Option<String> {
    return &event.data.entity_id;
}

/// Processes a "state_changed" event.
#[tracing::instrument]
fn state_changed(event: HassEvent) -> Option<()> {
    let entity_id = entity_id(&event).as_ref()?;
    let state_value = event.data.new_state.as_ref().map(|s| &s.state);

    let labels = metrics::StateLabels {
        entity_id: entity_id.to_string(),
    };

    // We expect most 'sensor' entities to have a value that can be parsed as f64,
    // which we attempt here.
    if let Some(val) = state_value.and_then(value_as_f64) {
        metrics::STATES.get_or_create(&labels).set(val);
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
    }
}
