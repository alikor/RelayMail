use relaymail_email::{validate, EmailError, MaxSize, RawEmail};

#[test]
fn rejects_missing_from() {
    let raw = RawEmail::from_slice(b"To: a@b.com\r\nSubject: x\r\n\r\nbody");
    let err = validate(&raw, MaxSize::default()).unwrap_err();
    assert!(matches!(err, EmailError::MissingHeader("From")));
}

#[test]
fn rejects_missing_recipients() {
    let raw = RawEmail::from_slice(b"From: a@b.com\r\nSubject: x\r\n\r\nbody");
    let err = validate(&raw, MaxSize::default()).unwrap_err();
    assert!(matches!(err, EmailError::NoRecipients));
}
