//! Raw MIME handling, validation, and redaction for RelayMail.
//!
//! Adapter-free: no AWS, no HTTP, no framework.

pub mod error;
pub mod metadata;
pub mod parse;
pub mod raw;
pub mod redaction;
pub mod validation;

pub use self::error::EmailError;
pub use self::metadata::{ContentType, EmailMetadata};
pub use self::parse::{parse_headers_only, Mailbox, ParsedHeaders};
pub use self::raw::{MaxSize, RawEmail};
pub use self::redaction::{redact_body_for_logs, redact_sensitive_headers};
pub use self::validation::validate;
