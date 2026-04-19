use relaymail_aws::S3EventParser;
use relaymail_runtime::pipeline::EventParser;

const DIRECT: &str = include_str!("../../../examples/events/s3-object-created.json");

#[test]
fn parses_direct_s3_event() {
    let parser = S3EventParser::new();
    let refs = parser.parse(DIRECT.as_bytes()).unwrap();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].object.bucket(), "relaymail-inbound-example");
    assert_eq!(refs[0].object.key(), "incoming/basic.eml");
    assert_eq!(refs[0].etag, "abc123def456");
    assert_eq!(refs[0].size, 287);
}
