use chrono::{DateTime, Utc};

/// Abstraction over reading the current wall-clock time.
///
/// Kept deliberately narrow: expose a single `now` method. Anything more
/// (monotonic time, sleep) belongs in a sibling trait.
pub trait Clock: Send + Sync + std::fmt::Debug {
    fn now(&self) -> DateTime<Utc>;
}
