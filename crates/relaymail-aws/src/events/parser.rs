use async_trait::async_trait;
use relaymail_core::ObjectId;
use relaymail_runtime::pipeline::{EventParseError, EventParser, ObjectRef};

use super::dispatch;

/// `EventParser` implementation that understands all three S3 envelope
/// shapes (direct, SNS-wrapped, EventBridge) plus the S3 `s3:TestEvent`
/// setup notification.
#[derive(Clone, Debug, Default)]
pub struct S3EventParser;

impl S3EventParser {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventParser for S3EventParser {
    fn parse(&self, bytes: &[u8]) -> Result<Vec<ObjectRef>, EventParseError> {
        let raw = std::str::from_utf8(bytes)
            .map_err(|_| EventParseError::InvalidJson("non-utf8 body".into()))?;
        let events = dispatch::parse(raw).map_err(map_err)?;
        Ok(events
            .into_iter()
            .map(|e| ObjectRef {
                object: ObjectId::new(e.bucket, e.key),
                etag: e.etag,
                size: e.size,
                event_time: e.event_time,
            })
            .collect())
    }
}

fn map_err(err: super::error::S3EventParseError) -> EventParseError {
    use super::error::S3EventParseError as E;
    match err {
        E::UnknownEnvelope => EventParseError::UnknownEnvelope,
        E::MissingField(name) => EventParseError::MissingField(name),
        E::InvalidJson(msg) => EventParseError::InvalidJson(msg),
    }
}
