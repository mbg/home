use hass_rs::WSEvent;

use crate::metrics;

/// Processes a web socket event from Home Assistant.
pub async fn process(message: WSEvent) {
    let event = message.event;

    metrics::EVENT_COUNTER
        .get_or_create(&metrics::EventLabels {
            event_type: event.event_type.clone(),
        })
        .inc();
}
