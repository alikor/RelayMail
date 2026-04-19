/// Produce a log-safe description of an email body — never its contents.
pub fn redact_body_for_logs(size_bytes: u64) -> String {
    format!("<redacted body, {size_bytes} bytes>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redaction_reports_size() {
        let out = redact_body_for_logs(1234);
        assert!(out.contains("1234"));
        assert!(out.contains("redacted"));
    }
}
