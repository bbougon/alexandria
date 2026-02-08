use serde_json::Value;

#[derive(Clone)]
pub struct Event {
    pub data: Value,
    pub event_type: String,
}

pub trait EventBus: Send + Sync {
    fn publish(&self, event: Event);
}

pub struct EventBusManager {
    pub event_bus: Box<dyn EventBus>,
}

impl EventBusManager {
    pub fn new(event_bus: Box<dyn EventBus>) -> Self {
        Self { event_bus }
    }
}
