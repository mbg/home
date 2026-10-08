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
    /// Some entities represent multiple metrics. If set,
    /// this names the sub-metric.
    pub metric: Option<String>,
    /// The HA domain of the entity.
    pub domain: String,
    /// The name of the entity, without the domain.
    pub name: String,
    /// The state of the entity, for enum-like states.
    pub state: Option<String>,

    /// The display friendly name of the entity.
    pub friendly_name: Option<String>,
    /// The class of device that this entity belongs to.
    /// For example, "enum".
    pub device_class: Option<String>,
    /// The class of state that this entity represents.
    /// For example, "measurement".
    pub state_class: Option<String>,
    /// The unit the state value is measured in.
    /// For example, "W"
    pub unit_of_measurement: Option<String>,
}

impl StateLabels {
    pub fn new(entity_id: String, domain: String, name: String) -> StateLabels {
        StateLabels {
            entity_id,
            metric: None,
            domain,
            name,
            state: None,
            friendly_name: None,
            device_class: None,
            state_class: None,
            unit_of_measurement: None,
        }
    }
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
