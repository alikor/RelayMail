//! Validation — does the raw email meet minimum delivery requirements?

pub(crate) mod limits;
pub(crate) mod recipients;
pub(crate) mod required_headers;

use crate::error::EmailError;
use crate::parse::{parse_headers_only, ParsedHeaders};
use crate::raw::{MaxSize, RawEmail};

/// Run all validation steps and return a parsed header view for downstream
/// metadata extraction.
pub fn validate(raw: &RawEmail, max_size: MaxSize) -> Result<ParsedHeaders, EmailError> {
    max_size.enforce(raw)?;
    let headers = parse_headers_only(raw.as_bytes())?;
    self::required_headers::check(&headers)?;
    self::recipients::check(&headers)?;
    self::limits::check(&headers)?;
    Ok(headers)
}
