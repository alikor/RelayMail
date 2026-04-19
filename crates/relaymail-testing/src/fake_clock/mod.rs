//! Deterministic clock backed by a cell of `DateTime<Utc>`.

pub(crate) mod clock;

pub use self::clock::FakeClock;
