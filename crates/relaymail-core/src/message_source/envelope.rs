use std::collections::HashMap;

use bytes::Bytes;

/// String attributes attached to an envelope (SQS message attributes, etc).
pub type EnvelopeAttributes = HashMap<String, String>;

/// An opaque message pulled from a [`super::contract::MessageSource`].
#[derive(Clone, Debug)]
pub struct RawEnvelope {
    id: String,
    body: Bytes,
    receipt_handle: String,
    attributes: EnvelopeAttributes,
}

impl RawEnvelope {
    pub fn new(id: impl Into<String>, body: Bytes, receipt_handle: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            body,
            receipt_handle: receipt_handle.into(),
            attributes: EnvelopeAttributes::new(),
        }
    }

    pub fn with_attributes(mut self, attrs: EnvelopeAttributes) -> Self {
        self.attributes = attrs;
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn body(&self) -> &Bytes {
        &self.body
    }

    pub fn receipt_handle(&self) -> &str {
        &self.receipt_handle
    }

    pub fn attributes(&self) -> &EnvelopeAttributes {
        &self.attributes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accessors_and_with_attributes() {
        let mut attrs = EnvelopeAttributes::new();
        attrs.insert("ApproximateReceiveCount".into(), "3".into());
        let env = RawEnvelope::new("msg-1", bytes::Bytes::from_static(b"body"), "hdl-1")
            .with_attributes(attrs);
        assert_eq!(env.id(), "msg-1");
        assert_eq!(env.body().as_ref(), b"body");
        assert_eq!(env.receipt_handle(), "hdl-1");
        assert_eq!(
            env.attributes()
                .get("ApproximateReceiveCount")
                .map(String::as_str),
            Some("3")
        );
    }
}
