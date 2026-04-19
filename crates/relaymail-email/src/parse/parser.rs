use mailparse::MailHeaderMap;

use super::addresses::parse_list;
use super::headers::ParsedHeaders;
use crate::error::EmailError;

/// Parse MIME headers only (cheap: we don't walk the body).
pub fn parse_headers_only(bytes: &[u8]) -> Result<ParsedHeaders, EmailError> {
    let (headers, _) = mailparse::parse_headers(bytes)?;
    let mut out = ParsedHeaders::default();
    if let Some(v) = headers.get_first_value("From") {
        out.from = parse_list(&v);
    }
    if let Some(v) = headers.get_first_value("To") {
        out.to = parse_list(&v);
    }
    if let Some(v) = headers.get_first_value("Cc") {
        out.cc = parse_list(&v);
    }
    if let Some(v) = headers.get_first_value("Bcc") {
        out.bcc = parse_list(&v);
    }
    out.subject = headers.get_first_value("Subject");
    out.message_id = headers.get_first_value("Message-ID");
    out.date = headers.get_first_value("Date");
    out.content_type = headers.get_first_value("Content-Type");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_headers() {
        let raw = b"From: a@b.com\r\nTo: c@d.com\r\nSubject: hi\r\n\r\nbody";
        let h = parse_headers_only(raw).unwrap();
        assert_eq!(h.from().len(), 1);
        assert_eq!(h.to().len(), 1);
        assert_eq!(h.subject(), Some("hi"));
    }
}
