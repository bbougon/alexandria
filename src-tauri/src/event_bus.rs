use serde_json::Value;
use tauri::{AppHandle, Emitter};

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

pub struct TauriEventBus {
    app: AppHandle,
}

impl EventBus for TauriEventBus {
    fn publish(&self, event: Event) {
        self.app.emit(&*event.event_type, event.data).unwrap();
    }
}

impl TauriEventBus {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }
}
