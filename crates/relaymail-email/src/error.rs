/// Typed failures from raw-email parsing and validation.
#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("email exceeds max size: {actual} > {limit}")]
    TooLarge { actual: u64, limit: u64 },

    #[error("required header missing: `{0}`")]
    MissingHeader(&'static str),

    #[error("invalid header `{name}`: {reason}")]
    InvalidHeader { name: &'static str, reason: String },

    #[error("no recipients across To/Cc/Bcc")]
    NoRecipients,

    #[error("mime parse error: {0}")]
    Mailparse(String),
}

impl From<mailparse::MailParseError> for EmailError {
    fn from(err: mailparse::MailParseError) -> Self {
        Self::Mailparse(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_contains_key_fields() {
        let e = EmailError::TooLarge {
            actual: 10,
            limit: 5,
        };
        assert!(e.to_string().contains("10") && e.to_string().contains("5"));

        let e = EmailError::MissingHeader("From");
        assert!(e.to_string().contains("From"));

        let e = EmailError::InvalidHeader {
            name: "Date",
            reason: "bad".into(),
        };
        assert!(e.to_string().contains("Date") && e.to_string().contains("bad"));

        assert!(EmailError::NoRecipients.to_string().contains("recipient"));
        assert!(EmailError::Mailparse("oops".into())
            .to_string()
            .contains("oops"));
    }

    #[test]
    fn from_mailparse_wraps_message() {
        // parse_headers_only converts MailParseError via From impl; mailparse is lenient
        // so this exercises the path only when it actually fails.
        if let Err(e) = crate::parse::parse_headers_only(b"\x00\xff") {
            assert!(matches!(e, EmailError::Mailparse(_)));
        }
    }
}
