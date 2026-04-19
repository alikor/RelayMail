use serde::{Deserialize, Serialize};
use std::fmt;

/// Tenant identifier.
///
/// Accepts ASCII alphanumerics plus `-` and `_`, between 1 and 128 chars.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TenantId(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TenantIdError {
    #[error("tenant id must be non-empty and <= 128 chars")]
    Length,
    #[error("tenant id contains invalid character: {0}")]
    InvalidChar(char),
}

impl TenantId {
    pub fn parse(raw: impl Into<String>) -> Result<Self, TenantIdError> {
        let value = raw.into();
        if value.is_empty() || value.len() > 128 {
            return Err(TenantIdError::Length);
        }
        if let Some(ch) = value.chars().find(|c| !is_valid_tenant_char(*c)) {
            return Err(TenantIdError::InvalidChar(ch));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_valid_tenant_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_'
}

impl fmt::Display for TenantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_basic_ids() {
        assert_eq!(TenantId::parse("acme").unwrap().as_str(), "acme");
        assert!(TenantId::parse("acme_corp-01").is_ok());
    }

    #[test]
    fn rejects_empty_and_long() {
        assert_eq!(TenantId::parse("").unwrap_err(), TenantIdError::Length);
        let long = "a".repeat(200);
        assert_eq!(TenantId::parse(long).unwrap_err(), TenantIdError::Length);
    }

    #[test]
    fn rejects_invalid_chars() {
        let err = TenantId::parse("has space").unwrap_err();
        assert!(matches!(err, TenantIdError::InvalidChar(' ')));
    }
}
