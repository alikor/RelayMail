use chrono::Utc;
use relaymail_delivery::SendResult;

use super::script::{SenderScript, Step};

/// Consume one step from the script; scripts that never exhaust clone their
/// baseline step.
pub(super) fn next_step(script: &mut SenderScript) -> Step {
    match script {
        SenderScript::AlwaysSuccess => Step::Success(SendResult::new("fake-msg-id", Utc::now())),
        SenderScript::AlwaysFail(err) => Step::Fail(clone_err(err)),
        SenderScript::Sequence(steps) => {
            if steps.is_empty() {
                Step::Success(SendResult::new("fake-msg-id", Utc::now()))
            } else {
                steps.remove(0)
            }
        }
    }
}

fn clone_err(err: &relaymail_delivery::SendError) -> relaymail_delivery::SendError {
    use relaymail_delivery::SendError::*;
    match err {
        Throttled(s) => Throttled(s.clone()),
        QuotaExceeded(s) => QuotaExceeded(s.clone()),
        Validation(s) => Validation(s.clone()),
        AuthenticationFailure(s) => AuthenticationFailure(s.clone()),
        InvalidRecipient(s) => InvalidRecipient(s.clone()),
        Suppressed(s) => Suppressed(s.clone()),
        Transient(s) => Transient(s.clone()),
        Permanent(s) => Permanent(s.clone()),
    }
}
