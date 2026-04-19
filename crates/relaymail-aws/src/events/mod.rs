//! S3 event-envelope parsers: direct, SNS-wrapped, and EventBridge.

pub(crate) mod decode;
pub(crate) mod direct;
pub(crate) mod dispatch;
pub(crate) mod error;
pub(crate) mod eventbridge;
pub(crate) mod parser;
pub(crate) mod sns;
pub(crate) mod types;

pub use self::error::S3EventParseError;
pub use self::parser::S3EventParser;
pub use self::types::S3ObjectEvent;
