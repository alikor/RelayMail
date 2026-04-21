use relaymail_email::{parse_headers_only, validate, EmailMetadata, MaxSize, RawEmail};

/// `X-SES-CONFIGURATION-SET` is the documented SES convention for
/// opting individual raw-MIME sends into a specific configuration set.
/// RelayMail surfaces the value through `ParsedHeaders` and
/// `EmailMetadata` so the SES adapter can apply it per-message.
#[test]
fn configuration_set_header_flows_through_metadata() {
    let raw = b"From: sender@example.com\r\n\
                To: recipient@example.com\r\n\
                Subject: config-set override\r\n\
                X-SES-CONFIGURATION-SET: jobvia-transactional-poc\r\n\
                \r\n\
                body";
    let headers = parse_headers_only(raw).unwrap();
    assert_eq!(
        headers.configuration_set(),
        Some("jobvia-transactional-poc")
    );
    let meta = EmailMetadata::from_headers(&headers, raw.len() as u64);
    assert_eq!(
        meta.configuration_set(),
        Some("jobvia-transactional-poc"),
        "metadata should carry the configuration set forward to the adapter"
    );
}

#[test]
fn no_header_means_no_override() {
    let raw = b"From: sender@example.com\r\n\
                To: recipient@example.com\r\n\
                Subject: no override\r\n\
                \r\n\
                body";
    let re = RawEmail::from_slice(raw);
    let headers = validate(&re, MaxSize::default()).unwrap();
    let meta = EmailMetadata::from_headers(&headers, re.len() as u64);
    assert_eq!(meta.configuration_set(), None);
}
