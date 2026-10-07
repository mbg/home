use std::sync::{LazyLock, atomic::AtomicU64};

use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{counter::Counter, family::Family, gauge::Gauge},
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

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct StateLabels {
    /// The ID of the entity, comprised of the domain and name.
    pub entity_id: String,
    /// The HA domain of the entity.
    pub domain: String,
    /// The name of the entity, without the domain.
    pub name: String,
}

/// A gauge for entity states.
pub static STATES: LazyLock<Family<StateLabels, Gauge<f64, AtomicU64>>> =
    std::sync::LazyLock::new(|| Family::<StateLabels, Gauge<f64, AtomicU64>>::default());

/// Initialises the metric registry and registers the Home Assistant metrics.
pub fn create() -> std::sync::Arc<Registry> {
    // Create the metric registry with the default prefix.
    let mut registry: Registry = <Registry>::with_prefix(DEFAULT_METRIC_PREFIX);

    registry.register(
        "events_total",
        "Home Assistant event counter.",
        EVENT_COUNTER.clone(),
    );
    registry.register("states", "Home Assistant entity states.", STATES.clone());

    return std::sync::Arc::new(registry);
}
