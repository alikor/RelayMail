use async_trait::async_trait;
use chrono::{DateTime, Utc};
use relaymail_core::ObjectId;

/// Normalized per-object reference yielded by the event parser.
#[derive(Clone, Debug)]
pub struct ObjectRef {
    pub object: ObjectId,
    pub etag: String,
    pub size: u64,
    pub event_time: DateTime<Utc>,
}

/// Adapter-agnostic error from event parsing.
#[derive(Debug, thiserror::Error)]
pub enum EventParseError {
    #[error("unknown envelope shape")]
    UnknownEnvelope,

    #[error("missing field: {0}")]
    MissingField(&'static str),

    #[error("invalid json: {0}")]
    InvalidJson(String),
}

/// Pluggable parser that turns an envelope body into a list of object refs.
#[async_trait]
pub trait EventParser: Send + Sync + std::fmt::Debug {
    fn parse(&self, bytes: &[u8]) -> Result<Vec<ObjectRef>, EventParseError>;
}
