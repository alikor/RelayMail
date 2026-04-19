use bytes::Bytes;
use relaymail_core::RawEnvelope;

/// Convenience builder for synthetic envelopes used in tests.
#[derive(Debug)]
pub struct FakeEnvelopeBuilder {
    id: String,
    receipt_handle: String,
    body: Bytes,
}

impl FakeEnvelopeBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            receipt_handle: "handle-0001".to_string(),
            body: Bytes::new(),
        }
    }

    pub fn with_body(mut self, body: impl Into<Bytes>) -> Self {
        self.body = body.into();
        self
    }

    pub fn with_receipt_handle(mut self, handle: impl Into<String>) -> Self {
        self.receipt_handle = handle.into();
        self
    }

    pub fn build(self) -> RawEnvelope {
        RawEnvelope::new(self.id, self.body, self.receipt_handle)
    }
}
