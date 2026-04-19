use relaymail_delivery::{SendError, SendResult};

/// Outcome the fake sender should produce for each call.
#[derive(Debug)]
pub enum Step {
    Success(SendResult),
    Fail(SendError),
}

/// Ordered list of steps the fake sender replays round-robin-once.
#[derive(Debug)]
pub enum SenderScript {
    AlwaysSuccess,
    AlwaysFail(SendError),
    Sequence(Vec<Step>),
}

impl SenderScript {
    pub fn sequence(steps: Vec<Step>) -> Self {
        Self::Sequence(steps)
    }

    pub fn always_throttled() -> Self {
        Self::AlwaysFail(SendError::Throttled("fake throttle".into()))
    }
}
