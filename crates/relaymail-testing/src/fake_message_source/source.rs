use std::collections::VecDeque;
use std::sync::Mutex;

use async_trait::async_trait;
use relaymail_core::message_source::{MessageSource, MessageSourceError};
use relaymail_core::RawEnvelope;

/// FIFO fake `MessageSource` used for pipeline tests.
#[derive(Debug, Default)]
pub struct FakeMessageSource {
    queue: Mutex<VecDeque<RawEnvelope>>,
    acks: Mutex<Vec<String>>,
    nacks: Mutex<Vec<String>>,
    extensions: Mutex<Vec<(String, u32)>>,
}

impl FakeMessageSource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue(&self, envelope: RawEnvelope) {
        self.queue.lock().expect("poisoned").push_back(envelope);
    }

    pub fn acks(&self) -> Vec<String> {
        self.acks.lock().expect("poisoned").clone()
    }

    pub fn nacks(&self) -> Vec<String> {
        self.nacks.lock().expect("poisoned").clone()
    }

    pub fn extensions(&self) -> Vec<(String, u32)> {
        self.extensions.lock().expect("poisoned").clone()
    }
}

#[async_trait]
impl MessageSource for FakeMessageSource {
    async fn receive(&self) -> Result<Vec<RawEnvelope>, MessageSourceError> {
        let mut guard = self.queue.lock().expect("poisoned");
        Ok(guard.drain(..).collect())
    }

    async fn ack(&self, envelope: &RawEnvelope) -> Result<(), MessageSourceError> {
        self.acks
            .lock()
            .expect("poisoned")
            .push(envelope.id().to_string());
        Ok(())
    }

    async fn nack(&self, envelope: &RawEnvelope) -> Result<(), MessageSourceError> {
        self.nacks
            .lock()
            .expect("poisoned")
            .push(envelope.id().to_string());
        Ok(())
    }

    async fn extend_visibility(
        &self,
        envelope: &RawEnvelope,
        seconds: u32,
    ) -> Result<(), MessageSourceError> {
        self.extensions
            .lock()
            .expect("poisoned")
            .push((envelope.id().to_string(), seconds));
        Ok(())
    }
}
