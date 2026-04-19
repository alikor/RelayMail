use super::*;

#[test]
fn transient_retries_then_dead_letters() {
    let p = DispositionPolicy::new(RetryLimits { max_attempts: 3 });
    assert_eq!(
        p.decide(ErrorClassification::Transient, AttemptCount::new(2)),
        DispositionDecision::Retry
    );
    assert_eq!(
        p.decide(ErrorClassification::Transient, AttemptCount::new(3)),
        DispositionDecision::DeadLetter
    );
}

#[test]
fn permanent_always_completes() {
    let p = DispositionPolicy::default();
    for c in [
        ErrorClassification::Validation,
        ErrorClassification::PermanentSender,
        ErrorClassification::PermanentRecipient,
        ErrorClassification::Unknown,
    ] {
        assert_eq!(
            p.decide(c, AttemptCount::first()),
            DispositionDecision::Complete
        );
    }
}
