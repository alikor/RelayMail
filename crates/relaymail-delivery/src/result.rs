use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

/// Successful-send details returned from the provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SendResult {
    provider_message_id: String,
    accepted_at: DateTime<Utc>,
    metadata: BTreeMap<String, String>,
}

impl SendResult {
    pub fn new(provider_message_id: impl Into<String>, accepted_at: DateTime<Utc>) -> Self {
        Self {
            provider_message_id: provider_message_id.into(),
            accepted_at,
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn provider_message_id(&self) -> &str {
        &self.provider_message_id
    }

    pub fn accepted_at(&self) -> DateTime<Utc> {
        self.accepted_at
    }

    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn accessors_and_with_metadata() {
        let now = Utc::now();
        let r = SendResult::new("msg-1", now).with_metadata("k", "v");
        assert_eq!(r.provider_message_id(), "msg-1");
        assert_eq!(r.accepted_at(), now);
        assert_eq!(r.metadata().get("k").map(String::as_str), Some("v"));
        assert!(r.metadata().get("missing").is_none());
    }
}
