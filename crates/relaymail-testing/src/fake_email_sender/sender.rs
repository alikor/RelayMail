use std::sync::Mutex;

use async_trait::async_trait;
use chrono::Utc;
use relaymail_delivery::{EmailSender, ProviderCapabilities, SendError, SendRequest, SendResult};

use super::script::{SenderScript, Step};

/// Fake `EmailSender` backed by a [`SenderScript`].
#[derive(Debug)]
pub struct FakeEmailSender {
    script: Mutex<SenderScript>,
    sent: Mutex<Vec<SendRequest>>,
}

impl FakeEmailSender {
    pub fn new(script: SenderScript) -> Self {
        Self {
            script: Mutex::new(script),
            sent: Mutex::new(Vec::new()),
        }
    }

    pub fn sent_requests(&self) -> Vec<SendRequest> {
        self.sent.lock().expect("poisoned").clone()
    }

    pub fn sent_count(&self) -> usize {
        self.sent.lock().expect("poisoned").len()
    }
}

#[async_trait]
impl EmailSender for FakeEmailSender {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            provider_label: "fake",
            max_message_bytes: u64::MAX,
            supports_raw_mime: true,
            supports_custom_headers: true,
        }
    }

    async fn send(&self, request: SendRequest) -> Result<SendResult, SendError> {
        let outcome = {
            let mut guard = self.script.lock().expect("poisoned");
            super::inspect::next_step(&mut guard)
        };
        self.sent.lock().expect("poisoned").push(request.clone());
        match outcome {
            Step::Success(r) => Ok(r),
            Step::Fail(e) => Err(e),
        }
        .map(attach_accepted_at)
    }
}

fn attach_accepted_at(mut result: SendResult) -> SendResult {
    if result.accepted_at().timestamp() == 0 {
        result = SendResult::new(result.provider_message_id(), Utc::now());
    }
    result
}
