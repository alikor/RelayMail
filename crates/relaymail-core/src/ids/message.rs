use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Internal RelayMail message identifier.
///
/// UUIDv7-backed so IDs sort roughly by creation time. Distinct from the
/// provider-supplied (e.g. SES) message id, which is captured in `SendResult`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MessageId(Uuid);

impl MessageId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_produces_unique_ids() {
        let a = MessageId::new();
        let b = MessageId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn from_uuid_and_as_uuid_round_trip() {
        let id = MessageId::new();
        let uuid = id.as_uuid();
        assert_eq!(MessageId::from_uuid(uuid), id);
    }

    #[test]
    fn default_and_display() {
        let id = MessageId::default();
        let s = id.to_string();
        assert_eq!(s.len(), 36, "UUID string is 36 chars");
        assert!(s.contains('-'));
    }
}
