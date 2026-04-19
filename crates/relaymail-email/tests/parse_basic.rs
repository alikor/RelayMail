use relaymail_email::{validate, ContentType, EmailMetadata, MaxSize, RawEmail};

const BASIC: &[u8] = include_bytes!("../../../examples/eml/basic.eml");

#[test]
fn validates_basic_fixture() {
    let raw = RawEmail::from_slice(BASIC);
    let headers = validate(&raw, MaxSize::default()).expect("basic fixture is valid");
    let meta = EmailMetadata::from_headers(&headers, raw.len() as u64);
    assert_eq!(meta.senders().len(), 1);
    assert_eq!(meta.senders()[0].address(), "sender@example.com");
    assert_eq!(meta.recipients().len(), 1);
    assert_eq!(meta.subject(), Some("Basic RelayMail test message"));
    assert_eq!(meta.content_type(), &ContentType::TextPlain);
    assert_eq!(meta.size_bytes(), raw.len() as u64);
}
