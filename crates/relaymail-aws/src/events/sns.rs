use serde::Deserialize;

use super::direct;
use super::error::S3EventParseError;
use super::types::S3ObjectEvent;

#[derive(Debug, Deserialize)]
struct Envelope {
    #[serde(rename = "Records")]
    records: Vec<Record>,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Sns")]
    sns: Sns,
}

#[derive(Debug, Deserialize)]
struct Sns {
    #[serde(rename = "Message")]
    message: String,
}

pub(crate) fn parse(raw: &str) -> Result<Vec<S3ObjectEvent>, S3EventParseError> {
    let env: Envelope = serde_json::from_str(raw)?;
    let mut out = Vec::new();
    for record in env.records {
        let inner = direct::parse(&record.sns.message)?;
        out.extend(inner);
    }
    Ok(out)
}
