use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct Event {
    pub data: Value,
    pub event_type: String,
}
pub trait EventBus: Send + Sync {
    fn publish(&self, event: Event);
}

#[derive(Clone)]
pub struct EventBusManager {
    pub event_bus: Arc<dyn EventBus>,
}

impl EventBusManager {
    pub fn new(event_bus: Arc<dyn EventBus>) -> Self {
        Self { event_bus }
    }

    pub fn publish<T: Serialize>(&self, event_type: &str, data: T) {
        self.event_bus.publish(Event {
            event_type: event_type.to_string(),
            data: serde_json::to_value(data).unwrap(),
        });
    }
}
