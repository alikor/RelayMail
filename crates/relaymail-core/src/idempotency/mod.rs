//! Idempotency — at-least-once semantics become effectively at-most-once.

pub(crate) mod error;
pub(crate) mod in_memory;
pub(crate) mod key;
pub(crate) mod store;

pub use self::error::IdempotencyError;
pub use self::in_memory::InMemoryIdempotencyStore;
pub use self::key::IdempotencyKey;
pub use self::store::{ClaimMetadata, ClaimOutcome, ClaimStatus, IdempotencyStore};
