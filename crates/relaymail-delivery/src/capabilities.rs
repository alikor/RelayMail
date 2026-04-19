/// Static metadata each provider reports. Used by operators and by the
/// runtime's pre-send checks (e.g. "does this raw message fit the provider
/// cap?").
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderCapabilities {
    pub provider_label: &'static str,
    pub max_message_bytes: u64,
    pub supports_raw_mime: bool,
    pub supports_custom_headers: bool,
}

impl ProviderCapabilities {
    pub const fn ses_v2() -> Self {
        Self {
            provider_label: "ses",
            max_message_bytes: 40 * 1024 * 1024,
            supports_raw_mime: true,
            supports_custom_headers: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ses_v2_has_expected_caps() {
        let c = ProviderCapabilities::ses_v2();
        assert_eq!(c.provider_label, "ses");
        assert_eq!(c.max_message_bytes, 40 * 1024 * 1024);
        assert!(c.supports_raw_mime);
        assert!(c.supports_custom_headers);
    }
}
