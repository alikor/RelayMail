//! Header-level MIME parsing.

pub(crate) mod addresses;
pub(crate) mod headers;
pub(crate) mod parser;

pub use self::addresses::Mailbox;
pub use self::headers::ParsedHeaders;
pub use self::parser::parse_headers_only;
