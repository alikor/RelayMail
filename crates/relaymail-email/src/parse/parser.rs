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
    // SES uses `X-SES-CONFIGURATION-SET` as the on-wire signal for
    // per-message configuration-set selection. When present, the SES
    // adapter uses it in place of the runtime default so producers can
    // route selected messages (e.g. transactional vs. marketing) to
    // different event destinations without bouncing the worker.
    out.configuration_set = headers
        .get_first_value("X-SES-CONFIGURATION-SET")
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
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
        assert_eq!(h.configuration_set(), None);
    }

    #[test]
    fn extracts_configuration_set_header() {
        let raw = b"From: a@b.com\r\nTo: c@d.com\r\n\
                    X-SES-CONFIGURATION-SET: my-config-set\r\n\
                    Subject: hi\r\n\r\nbody";
        let h = parse_headers_only(raw).unwrap();
        assert_eq!(h.configuration_set(), Some("my-config-set"));
    }

    #[test]
    fn configuration_set_trimmed_and_empty_rejected() {
        let raw = b"From: a@b.com\r\nTo: c@d.com\r\n\
                    X-SES-CONFIGURATION-SET:    \r\n\
                    Subject: hi\r\n\r\nbody";
        let h = parse_headers_only(raw).unwrap();
        assert_eq!(h.configuration_set(), None);
    }
}
