use crate::error::EmailError;
use crate::parse::ParsedHeaders;

/// A From header is mandatory. Date and Message-ID are recommended by
/// RFC 5322 but not hard-enforced here because SES will add them if missing.
pub(crate) fn check(headers: &ParsedHeaders) -> Result<(), EmailError> {
    if headers.from().is_empty() {
        return Err(EmailError::MissingHeader("From"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_headers_only;

    #[test]
    fn rejects_missing_from() {
        let raw = b"To: a@b.com\r\nSubject: x\r\n\r\n";
        let h = parse_headers_only(raw).unwrap();
        assert!(matches!(check(&h), Err(EmailError::MissingHeader("From"))));
    }

    #[test]
    fn accepts_with_from() {
        let raw = b"From: a@b.com\r\nTo: c@d.com\r\n\r\n";
        let h = parse_headers_only(raw).unwrap();
        assert!(check(&h).is_ok());
    }
}
