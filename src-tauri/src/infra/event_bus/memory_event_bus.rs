#[cfg(test)]
use crate::event_bus::{Event, EventBus};
#[cfg(test)]
use std::sync::Arc;

#[cfg(test)]
pub struct MemoryEventBus {
    pub events: parking_lot::Mutex<Vec<Event>>,
}

#[cfg(test)]
impl MemoryEventBus {
    pub fn new() -> Self {
        Self {
            events: parking_lot::Mutex::new(Vec::new()),
        }
    }
}

#[cfg(test)]
impl EventBus for MemoryEventBus {
    fn publish(&self, event: Event) {
        self.events.lock().push(event);
    }
}

#[cfg(test)]
impl EventBus for Arc<MemoryEventBus> {
    fn publish(&self, event: Event) {
        self.events.lock().push(event);
    }
}
