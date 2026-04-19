use crate::error::EmailError;
use crate::parse::ParsedHeaders;

pub(crate) fn check(headers: &ParsedHeaders) -> Result<(), EmailError> {
    if headers.recipient_count() == 0 {
        return Err(EmailError::NoRecipients);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_headers_only;

    #[test]
    fn accepts_to_only() {
        let raw = b"From: a@b.com\r\nTo: c@d.com\r\n\r\n";
        assert!(check(&parse_headers_only(raw).unwrap()).is_ok());
    }

    #[test]
    fn accepts_bcc_only() {
        let raw = b"From: a@b.com\r\nBcc: c@d.com\r\n\r\n";
        assert!(check(&parse_headers_only(raw).unwrap()).is_ok());
    }

    #[test]
    fn rejects_empty_recipients() {
        let raw = b"From: a@b.com\r\n\r\n";
        assert!(matches!(
            check(&parse_headers_only(raw).unwrap()),
            Err(EmailError::NoRecipients)
        ));
    }
}
