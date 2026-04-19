//! Abstraction over the queue / event source that feeds the worker.

pub(crate) mod contract;
pub(crate) mod envelope;
pub(crate) mod error;

pub use self::contract::MessageSource;
pub use self::envelope::{EnvelopeAttributes, RawEnvelope};
pub use self::error::MessageSourceError;
