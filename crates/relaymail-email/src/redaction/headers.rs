use crate::parse::ParsedHeaders;

/// Hide secrets + full recipient addresses from a parsed header set.
#[derive(Debug, Eq, PartialEq)]
pub struct RedactedHeaders {
    pub from_domains: Vec<String>,
    pub to_count: usize,
    pub cc_count: usize,
    pub bcc_count: usize,
    pub subject_preview: Option<String>,
}

pub fn redact_sensitive_headers(headers: &ParsedHeaders) -> RedactedHeaders {
    let from_domains = headers
        .from()
        .iter()
        .filter_map(|m| m.address().rsplit_once('@').map(|(_, d)| d.to_string()))
        .collect();
    let subject_preview = headers.subject().map(|s| preview(s, 32));
    RedactedHeaders {
        from_domains,
        to_count: headers.to().len(),
        cc_count: headers.cc().len(),
        bcc_count: headers.bcc().len(),
        subject_preview,
    }
}

/// Reduce an email address to `first-char + @ + domain` for safe logging.
pub fn redact_recipient(address: &str) -> String {
    match address.rsplit_once('@') {
        Some((local, domain)) => {
            let first = local.chars().next().unwrap_or('?');
            format!("{first}***@{domain}")
        }
        None => "<invalid>".to_string(),
    }
}

fn preview(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipient_masks_local_part() {
        assert_eq!(redact_recipient("alice@example.com"), "a***@example.com");
        assert_eq!(redact_recipient("not-an-email"), "<invalid>");
    }

    #[test]
    fn preview_short_value_unchanged() {
        assert_eq!(super::preview("Hi", 32), "Hi");
        assert_eq!(super::preview(&"A".repeat(32), 32), "A".repeat(32));
    }

    #[test]
    fn preview_truncates_long_value() {
        let long = "A".repeat(40);
        let truncated = super::preview(&long, 32);
        assert!(truncated.ends_with('…'));
        assert!(truncated.len() < long.len());
    }

    #[test]
    fn redact_sensitive_headers_counts_recipients() {
        use crate::parse::parse_headers_only;
        let raw = b"From: alice@example.com\r\nTo: bob@b.com, carol@c.com\r\nCc: dan@d.com\r\n\r\n";
        let headers = parse_headers_only(raw).unwrap();
        let r = redact_sensitive_headers(&headers);
        assert_eq!(r.from_domains, vec!["example.com"]);
        assert_eq!(r.to_count, 2);
        assert_eq!(r.cc_count, 1);
        assert!(r.subject_preview.is_none());
    }

    #[test]
    fn redact_sensitive_headers_truncates_subject() {
        use crate::parse::parse_headers_only;
        let long_subj = "A".repeat(40);
        let raw = format!("From: a@b.com\r\nTo: c@d.com\r\nSubject: {long_subj}\r\n\r\n");
        let headers = parse_headers_only(raw.as_bytes()).unwrap();
        let r = redact_sensitive_headers(&headers);
        assert!(r.subject_preview.as_deref().unwrap().ends_with('…'));
    }
}
