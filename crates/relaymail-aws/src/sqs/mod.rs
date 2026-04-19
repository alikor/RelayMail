//! SQS-backed `MessageSource` implementation.

pub(crate) mod consumer;
pub(crate) mod error_map;
pub(crate) mod impl_message_source;

pub use self::consumer::{SqsConsumer, SqsConsumerConfig};
