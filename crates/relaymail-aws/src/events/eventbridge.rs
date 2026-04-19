use chrono::DateTime;
use serde::Deserialize;

use super::decode::url_decode_key;
use super::error::S3EventParseError;
use super::types::S3ObjectEvent;

#[derive(Debug, Deserialize)]
struct Envelope {
    time: String,
    detail: Detail,
}

#[derive(Debug, Deserialize)]
struct Detail {
    bucket: Bucket,
    object: Object,
}

#[derive(Debug, Deserialize)]
struct Bucket {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Object {
    key: String,
    size: Option<u64>,
    etag: Option<String>,
    sequencer: Option<String>,
}

pub(crate) fn parse(raw: &str) -> Result<Vec<S3ObjectEvent>, S3EventParseError> {
    let env: Envelope = serde_json::from_str(raw)?;
    let at = DateTime::parse_from_rfc3339(&env.time)
        .map_err(|_| S3EventParseError::MissingField("time"))?
        .with_timezone(&chrono::Utc);
    Ok(vec![S3ObjectEvent {
        bucket: env.detail.bucket.name,
        key: url_decode_key(&env.detail.object.key),
        etag: env.detail.object.etag.unwrap_or_default(),
        size: env.detail.object.size.unwrap_or(0),
        sequencer: env.detail.object.sequencer,
        event_time: at,
    }])
}
