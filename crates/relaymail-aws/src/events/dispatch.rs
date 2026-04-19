use serde_json::Value;

use super::{direct, error::S3EventParseError, eventbridge, sns, types::S3ObjectEvent};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Direct,
    Sns,
    EventBridge,
    S3TestEvent,
    Unknown,
}

pub(crate) fn parse(raw: &str) -> Result<Vec<S3ObjectEvent>, S3EventParseError> {
    let value: Value = serde_json::from_str(raw)?;
    match detect(&value) {
        Kind::S3TestEvent => Ok(Vec::new()),
        Kind::Direct => direct::parse(raw),
        Kind::Sns => sns::parse(raw),
        Kind::EventBridge => eventbridge::parse(raw),
        Kind::Unknown => Err(S3EventParseError::UnknownEnvelope),
    }
}

fn detect(value: &Value) -> Kind {
    if value.get("Event").and_then(Value::as_str) == Some("s3:TestEvent") {
        return Kind::S3TestEvent;
    }
    if value.get("detail-type").is_some() && value.get("detail").is_some() {
        return Kind::EventBridge;
    }
    if let Some(records) = value.get("Records").and_then(Value::as_array) {
        if let Some(first) = records.first() {
            if first.get("Sns").is_some() {
                return Kind::Sns;
            }
            if first.get("s3").is_some() {
                return Kind::Direct;
            }
        }
    }
    Kind::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_test_event() {
        let v: Value = serde_json::from_str(r#"{"Event":"s3:TestEvent"}"#).unwrap();
        assert_eq!(detect(&v), Kind::S3TestEvent);
    }

    #[test]
    fn detects_unknown() {
        let v: Value = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(detect(&v), Kind::Unknown);
    }
}
