use relaymail_core::config::parse_csv_list;

use super::flat::FlatConfig;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct PollingConfig {
    pub enabled: bool,
    pub interval_seconds: u64,
    pub buckets: Vec<String>,
    pub prefixes: Vec<String>,
}

impl PollingConfig {
    pub(crate) fn from_flat(flat: &FlatConfig) -> Self {
        Self {
            enabled: flat.polling_mode_enabled.unwrap_or(false),
            interval_seconds: flat.polling_interval_seconds.unwrap_or(60),
            buckets: flat
                .polling_buckets
                .as_deref()
                .map(parse_csv_list)
                .unwrap_or_default(),
            prefixes: flat
                .polling_prefixes
                .as_deref()
                .map(parse_csv_list)
                .unwrap_or_default(),
        }
    }
}
