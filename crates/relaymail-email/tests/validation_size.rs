use relaymail_email::{validate, EmailError, MaxSize, RawEmail};

#[test]
fn rejects_over_limit() {
    let raw = RawEmail::from_slice(b"From: a@b.com\r\nTo: c@d.com\r\n\r\nbody");
    let err = validate(&raw, MaxSize::new(8)).unwrap_err();
    assert!(matches!(err, EmailError::TooLarge { .. }));
}
