//! Canonical fixtures for tests: fixtures live in `examples/` under the repo
//! root so deploy artifacts can reuse them.

pub(crate) mod eml;
pub(crate) mod envelopes;
pub(crate) mod events;

pub use self::eml::{basic_eml, multipart_eml};
pub use self::envelopes::{direct_envelope, eventbridge_envelope, sns_envelope};
pub use self::events::{direct_event, eventbridge_event, sns_event};
