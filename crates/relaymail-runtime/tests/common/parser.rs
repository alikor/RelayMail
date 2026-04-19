use async_trait::async_trait;
use chrono::Utc;
use relaymail_core::ObjectId;
use relaymail_runtime::pipeline::{EventParseError, EventParser, ObjectRef};

pub const BUCKET: &str = "relaymail-inbound-example";
pub const KEY: &str = "incoming/a.eml";

#[derive(Debug)]
pub struct StaticEventParser {
    pub refs: Vec<ObjectRef>,
}

#[async_trait]
impl EventParser for StaticEventParser {
    fn parse(&self, _: &[u8]) -> Result<Vec<ObjectRef>, EventParseError> {
        Ok(self.refs.clone())
    }
}

pub fn object_ref() -> ObjectRef {
    ObjectRef {
        object: ObjectId::new(BUCKET, KEY),
        etag: "etag-v1".into(),
        size: 287,
        event_time: Utc::now(),
    }
}
