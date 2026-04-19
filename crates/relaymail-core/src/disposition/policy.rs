use super::attempt::AttemptCount;
use super::classification::ErrorClassification;

/// Decision returned by [`DispositionPolicy::decide`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DispositionDecision {
    /// Return the message to the source for another attempt.
    Retry,

    /// The attempt cap was hit — send to DLQ (i.e. ack + tag failed).
    DeadLetter,

    /// Permanent failure — ack the message and tag the object as failed.
    Complete,

    /// Safe to drop (e.g. unsupported extension, unknown envelope) — ack.
    Drop,
}

/// Tunables for [`DispositionPolicy`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryLimits {
    pub max_attempts: u32,
}

impl Default for RetryLimits {
    fn default() -> Self {
        Self { max_attempts: 5 }
    }
}

/// Pure decision function that turns an error classification and attempt
/// count into a disposition.
#[derive(Clone, Copy, Debug, Default)]
pub struct DispositionPolicy {
    limits: RetryLimits,
}

impl DispositionPolicy {
    pub fn new(limits: RetryLimits) -> Self {
        Self { limits }
    }

    pub fn decide(
        &self,
        classification: ErrorClassification,
        attempt: AttemptCount,
    ) -> DispositionDecision {
        match classification {
            ErrorClassification::Transient => {
                if attempt.is_exhausted(self.limits.max_attempts) {
                    DispositionDecision::DeadLetter
                } else {
                    DispositionDecision::Retry
                }
            }
            ErrorClassification::Validation
            | ErrorClassification::PermanentRecipient
            | ErrorClassification::PermanentSender
            | ErrorClassification::Unknown => DispositionDecision::Complete,
        }
    }

    pub fn limits(&self) -> RetryLimits {
        self.limits
    }
}

#[cfg(test)]
#[path = "policy_tests.rs"]
mod tests;
