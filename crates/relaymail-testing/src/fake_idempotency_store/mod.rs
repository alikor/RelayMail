//! Thin test-only wrapper re-exporting the in-memory implementation.

pub(crate) mod store;

pub use self::store::FakeIdempotencyStore;
