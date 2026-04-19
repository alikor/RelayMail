use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Stable, serde-friendly wall-clock timestamp used at domain boundaries.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Instant(DateTime<Utc>);

impl Instant {
    pub fn new(inner: DateTime<Utc>) -> Self {
        Self(inner)
    }

    pub fn as_utc(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn to_rfc3339(self) -> String {
        self.0.to_rfc3339()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_rfc3339() {
        let now = Utc::now();
        let t = Instant::new(now);
        assert!(t.to_rfc3339().contains('T'));
        assert_eq!(t.as_utc(), now);
    }
}
