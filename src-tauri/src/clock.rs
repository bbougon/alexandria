use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::sync::Arc;

pub trait Clock: Send + Sync + 'static {
    fn now(&self) -> DateTime<Utc>;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[cfg(test)]
pub struct StaticClock {
    now: DateTime<Utc>,
}

#[cfg(test)]
impl StaticClock {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self { now }
    }
}

#[cfg(test)]
impl Clock for StaticClock {
    fn now(&self) -> DateTime<Utc> {
        self.now
    }
}

static CLOCK: Lazy<RwLock<Arc<dyn Clock>>> = Lazy::new(|| RwLock::new(Arc::new(SystemClock)));

pub fn clock() -> Arc<dyn Clock> {
    Arc::clone(&CLOCK.read())
}

#[cfg(test)]
pub fn set_clock(new_clock: Arc<dyn Clock>) {
    *CLOCK.write() = new_clock;
}

#[cfg(test)]
pub struct ClockGuard {
    previous: Arc<dyn Clock>,
}

#[cfg(test)]
impl Drop for ClockGuard {
    fn drop(&mut self) {
        *CLOCK.write() = Arc::clone(&self.previous);
    }
}

#[cfg(test)]
pub fn with_clock_temp(new_clock: Arc<dyn Clock>) -> ClockGuard {
    let previous = clock();
    set_clock(new_clock);
    ClockGuard { previous }
}

#[cfg(test)]
pub fn with_static_clock(now: DateTime<Utc>) -> ClockGuard {
    with_clock_temp(Arc::new(StaticClock::new(now)))
}
