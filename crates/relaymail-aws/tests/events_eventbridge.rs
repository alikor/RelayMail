use relaymail_aws::S3EventParser;
use relaymail_runtime::pipeline::EventParser;

const EB: &str = include_str!("../../../examples/events/eventbridge-s3-event.json");

#[test]
fn parses_eventbridge_s3_event() {
    let parser = S3EventParser::new();
    let refs = parser.parse(EB.as_bytes()).unwrap();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].object.key(), "incoming/eventbridge.eml");
    assert_eq!(refs[0].size, 640);
}
