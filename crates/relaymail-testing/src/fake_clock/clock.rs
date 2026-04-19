use std::sync::Mutex;

use chrono::{DateTime, Duration, TimeZone, Utc};
use relaymail_core::Clock;

/// Fake clock usable via `Arc<dyn Clock>`.
#[derive(Debug)]
pub struct FakeClock {
    inner: Mutex<DateTime<Utc>>,
}

impl FakeClock {
    pub fn new(at: DateTime<Utc>) -> Self {
        Self {
            inner: Mutex::new(at),
        }
    }

    pub fn epoch() -> Self {
        Self::new(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).single().unwrap())
    }

    pub fn advance(&self, by: Duration) {
        if let Ok(mut guard) = self.inner.lock() {
            *guard += by;
        }
    }
}

impl Clock for FakeClock {
    fn now(&self) -> DateTime<Utc> {
        *self.inner.lock().expect("poisoned clock lock")
    }
}
