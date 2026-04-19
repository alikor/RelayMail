use relaymail_email::{validate, ContentType, EmailMetadata, MaxSize, RawEmail};

const MULTI: &[u8] = include_bytes!("../../../examples/eml/multipart-with-attachment.eml");

#[test]
fn multipart_has_cc_and_preserves_boundary() {
    let raw = RawEmail::from_slice(MULTI);
    let headers = validate(&raw, MaxSize::default()).expect("multipart fixture is valid");
    let meta = EmailMetadata::from_headers(&headers, raw.len() as u64);
    assert_eq!(meta.recipients().len(), 2, "To + Cc");
    assert_eq!(meta.content_type(), &ContentType::MultipartMixed);
    assert!(meta.message_id().unwrap().contains("multipart-test"));
    // The raw bytes must not be rewritten by our handling.
    let needle = br#"boundary="relaymail-boundary-001""#;
    assert!(raw.as_bytes().windows(needle.len()).any(|w| w == needle));
}
