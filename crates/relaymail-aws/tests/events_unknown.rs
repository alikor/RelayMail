use relaymail_aws::S3EventParser;
use relaymail_runtime::pipeline::{EventParseError, EventParser};

#[test]
fn unknown_envelope_errors() {
    let parser = S3EventParser::new();
    let err = parser.parse(b"{}").unwrap_err();
    assert!(matches!(err, EventParseError::UnknownEnvelope));
}

#[test]
fn test_event_returns_empty() {
    let parser = S3EventParser::new();
    let refs = parser.parse(br#"{"Event":"s3:TestEvent"}"#).unwrap();
    assert!(refs.is_empty());
}
