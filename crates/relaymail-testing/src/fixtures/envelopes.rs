use bytes::Bytes;
use relaymail_core::RawEnvelope;

use super::events::{direct_event, eventbridge_event, sns_event};

pub fn direct_envelope() -> RawEnvelope {
    RawEnvelope::new("env-direct", Bytes::from(direct_event()), "handle-direct")
}

pub fn sns_envelope() -> RawEnvelope {
    RawEnvelope::new("env-sns", Bytes::from(sns_event()), "handle-sns")
}

pub fn eventbridge_envelope() -> RawEnvelope {
    RawEnvelope::new("env-eb", Bytes::from(eventbridge_event()), "handle-eb")
}
