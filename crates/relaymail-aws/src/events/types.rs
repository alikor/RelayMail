use chrono::{DateTime, Utc};

/// Normalized S3 object-created event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S3ObjectEvent {
    pub bucket: String,
    pub key: String,
    pub etag: String,
    pub size: u64,
    pub sequencer: Option<String>,
    pub event_time: DateTime<Utc>,
}
