use std::sync::LazyLock;

use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{counter::Counter, family::Family},
    registry::Registry,
};

use crate::config::DEFAULT_METRIC_PREFIX;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct EventLabels {
    /// The event type identifier.
    pub event_type: String,
}

/// A counter for all events.
pub static EVENT_COUNTER: LazyLock<Family<EventLabels, Counter>> =
    std::sync::LazyLock::new(|| Family::<EventLabels, Counter>::default());

/// Initialises the metric registry and registers the Home Assistant metrics.
pub fn create() -> std::sync::Arc<Registry> {
    // Create the metric registry with the default prefix.
    let mut registry: Registry = <Registry>::with_prefix(DEFAULT_METRIC_PREFIX);

    registry.register(
        "events_total",
        "Home Assistant event counter.",
        EVENT_COUNTER.clone(),
    );

    return std::sync::Arc::new(registry);
}
