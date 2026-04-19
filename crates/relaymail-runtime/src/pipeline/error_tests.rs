use relaymail_core::{
    idempotency::IdempotencyError, message_source::MessageSourceError,
    object_store::ObjectStoreError, ErrorClassification,
};
use relaymail_email::EmailError;

use super::{EventParseError, StageError};

#[test]
fn event_parse_and_validate_classify_as_validation() {
    let ep = StageError::EventParse(EventParseError::UnknownEnvelope);
    assert_eq!(ep.classification(), ErrorClassification::Validation);
    let v = StageError::Validate(EmailError::NoRecipients);
    assert_eq!(v.classification(), ErrorClassification::Validation);
}

#[test]
fn fetch_variants_classify_correctly() {
    assert_eq!(
        StageError::Fetch(ObjectStoreError::NotFound("k".into())).classification(),
        ErrorClassification::Validation
    );
    assert_eq!(
        StageError::Fetch(ObjectStoreError::TooLarge {
            actual: 2,
            limit: 1
        })
        .classification(),
        ErrorClassification::Validation
    );
    assert_eq!(
        StageError::Fetch(ObjectStoreError::PermissionDenied("d".into())).classification(),
        ErrorClassification::PermanentSender
    );
    assert_eq!(
        StageError::Fetch(ObjectStoreError::Transient("t".into())).classification(),
        ErrorClassification::Transient
    );
    assert_eq!(
        StageError::Fetch(ObjectStoreError::Permanent("p".into())).classification(),
        ErrorClassification::PermanentSender
    );
}

#[test]
fn claim_and_source_variants_classify_correctly() {
    assert_eq!(
        StageError::Claim(IdempotencyError::Transient("t".into())).classification(),
        ErrorClassification::Transient
    );
    assert_eq!(
        StageError::Claim(IdempotencyError::Permanent("p".into())).classification(),
        ErrorClassification::PermanentSender
    );
    assert_eq!(
        StageError::Source(MessageSourceError::Transient("t".into())).classification(),
        ErrorClassification::Transient
    );
    assert_eq!(
        StageError::Source(MessageSourceError::InvalidEnvelope("b".into())).classification(),
        ErrorClassification::PermanentSender
    );
}
