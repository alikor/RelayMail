use relaymail_core::disposition::{
    AttemptCount, DispositionDecision, DispositionPolicy, ErrorClassification, RetryLimits,
};

#[test]
fn matrix_matches_spec() {
    let policy = DispositionPolicy::new(RetryLimits { max_attempts: 3 });
    let cases: &[(ErrorClassification, u32, DispositionDecision)] = &[
        (
            ErrorClassification::Transient,
            1,
            DispositionDecision::Retry,
        ),
        (
            ErrorClassification::Transient,
            2,
            DispositionDecision::Retry,
        ),
        (
            ErrorClassification::Transient,
            3,
            DispositionDecision::DeadLetter,
        ),
        (
            ErrorClassification::Transient,
            5,
            DispositionDecision::DeadLetter,
        ),
        (
            ErrorClassification::Validation,
            1,
            DispositionDecision::Complete,
        ),
        (
            ErrorClassification::PermanentSender,
            1,
            DispositionDecision::Complete,
        ),
        (
            ErrorClassification::PermanentRecipient,
            2,
            DispositionDecision::Complete,
        ),
        (
            ErrorClassification::Unknown,
            1,
            DispositionDecision::Complete,
        ),
    ];
    for (class, attempt, expected) in cases {
        let got = policy.decide(*class, AttemptCount::new(*attempt));
        assert_eq!(got, *expected, "class={class:?} attempt={attempt}");
    }
}

#[test]
fn default_limits_are_sensible() {
    let policy = DispositionPolicy::default();
    assert_eq!(policy.limits().max_attempts, 5);
}
