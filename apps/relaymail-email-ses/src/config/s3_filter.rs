use relaymail_core::config::parse_csv_list;

use super::flat::FlatConfig;

#[derive(Clone, Debug)]
pub(crate) struct S3FilterConfig {
    pub bucket_allowlist: Vec<String>,
    pub prefix_allowlist: Vec<String>,
    pub supported_extensions: Vec<String>,
    pub max_email_bytes: u64,
}

impl S3FilterConfig {
    pub(crate) fn from_flat(flat: &FlatConfig) -> Self {
        let bucket_allowlist = flat
            .s3_bucket_allowlist
            .as_deref()
            .map(parse_csv_list)
            .unwrap_or_default();
        let prefix_allowlist = flat
            .s3_prefix_allowlist
            .as_deref()
            .map(parse_csv_list)
            .unwrap_or_default();
        let extensions = flat
            .supported_extensions
            .as_deref()
            .map(parse_csv_list)
            .unwrap_or_else(|| vec![".eml".to_string(), ".emi".to_string()]);
        Self {
            bucket_allowlist,
            prefix_allowlist,
            supported_extensions: extensions,
            max_email_bytes: flat.max_email_bytes.unwrap_or(10 * 1024 * 1024),
        }
    }
}
