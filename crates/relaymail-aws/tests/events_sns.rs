use relaymail_aws::S3EventParser;
use relaymail_runtime::pipeline::EventParser;

const SNS: &str = include_str!("../../../examples/events/sns-wrapped-s3-event.json");

#[test]
fn parses_sns_wrapped_s3_event() {
    let parser = S3EventParser::new();
    let refs = parser.parse(SNS.as_bytes()).unwrap();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].object.key(), "incoming/sns-wrapped.eml");
    assert_eq!(refs[0].size, 512);
}
