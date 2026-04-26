use relaymail_core::ErrorClassification;
use relaymail_delivery::SendError;

#[test]
fn classification_matrix() {
    let cases: Vec<(SendError, ErrorClassification)> = vec![
        (
            SendError::Throttled("x".into()),
            ErrorClassification::Transient,
        ),
        (
            SendError::Transient("x".into()),
            ErrorClassification::Transient,
        ),
        (
            SendError::QuotaExceeded("x".into()),
            ErrorClassification::PermanentSender,
        ),
        (
            SendError::Permanent("x".into()),
            ErrorClassification::PermanentSender,
        ),
        (
            SendError::AuthenticationFailure("x".into()),
            ErrorClassification::PermanentSender,
        ),
        (
            SendError::InvalidRecipient("x".into()),
            ErrorClassification::PermanentRecipient,
        ),
        (
            SendError::Suppressed("x".into()),
            ErrorClassification::PermanentRecipient,
        ),
        (
            SendError::Validation("x".into()),
            ErrorClassification::Validation,
        ),
    ];
    for (err, expected) in cases {
        assert_eq!(err.classify(), expected, "err={err}");
    }
}
