//! Metadata extraction for post-validation pipeline use.

pub(crate) mod content_type;
pub(crate) mod extract;

pub use self::content_type::ContentType;
pub use self::extract::EmailMetadata;
