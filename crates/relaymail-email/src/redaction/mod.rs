//! Redaction helpers — produce log-safe views that never leak bodies or
//! full recipient addresses.

pub(crate) mod body;
pub(crate) mod headers;

pub use self::body::redact_body_for_logs;
pub use self::headers::{redact_recipient, redact_sensitive_headers};
