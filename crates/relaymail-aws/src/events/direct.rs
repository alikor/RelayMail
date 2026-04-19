use chrono::DateTime;
use serde::Deserialize;

use super::decode::url_decode_key;
use super::error::S3EventParseError;
use super::types::S3ObjectEvent;

#[derive(Debug, Deserialize)]
struct Envelope {
    #[serde(rename = "Records")]
    records: Vec<Record>,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "eventTime")]
    event_time: String,
    s3: S3Block,
}

#[derive(Debug, Deserialize)]
struct S3Block {
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
    #[serde(rename = "eTag")]
    etag: Option<String>,
    sequencer: Option<String>,
}

pub(crate) fn parse(raw: &str) -> Result<Vec<S3ObjectEvent>, S3EventParseError> {
    let env: Envelope = serde_json::from_str(raw)?;
    let mut out = Vec::with_capacity(env.records.len());
    for record in env.records {
        let at = DateTime::parse_from_rfc3339(&record.event_time)
            .map_err(|_| S3EventParseError::MissingField("eventTime"))?
            .with_timezone(&chrono::Utc);
        out.push(S3ObjectEvent {
            bucket: record.s3.bucket.name,
            key: url_decode_key(&record.s3.object.key),
            etag: record.s3.object.etag.unwrap_or_default(),
            size: record.s3.object.size.unwrap_or(0),
            sequencer: record.s3.object.sequencer,
            event_time: at,
        });
    }
    Ok(out)
}
