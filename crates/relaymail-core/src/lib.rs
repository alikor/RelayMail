//! Shared domain types and capability traits for RelayMail.
//!
//! This crate is adapter-free: it depends only on standard libraries, `tokio`
//! primitives, and `serde`. AWS, HTTP, email parsing, and provider code must
//! live in sibling crates so the domain types stay stable and portable.

#![deny(missing_debug_implementations)]

pub mod clock;
pub mod config;
pub mod disposition;
pub mod errors;
pub mod idempotency;
pub mod ids;
pub mod message_source;
pub mod object_store;
pub mod time;

pub use crate::clock::{Clock, SystemClock};
pub use crate::disposition::{
    AttemptCount, DispositionDecision, DispositionPolicy, ErrorClassification,
};
pub use crate::errors::DomainError;
pub use crate::idempotency::{
    ClaimMetadata, ClaimOutcome, ClaimStatus, IdempotencyError, IdempotencyKey, IdempotencyStore,
    InMemoryIdempotencyStore,
};
pub use crate::ids::{MessageId, ObjectId, TenantId};
pub use crate::message_source::{MessageSourceError, RawEnvelope};
pub use crate::object_store::{ObjectMetadata, ObjectStoreError, TagSet};
pub use crate::time::Instant;
