use crate::event_bus::{Event, EventBus};
use tauri::{AppHandle, Emitter};

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
