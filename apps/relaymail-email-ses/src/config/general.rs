use super::flat::FlatConfig;

#[derive(Clone, Debug)]
pub(crate) struct GeneralConfig {
    pub service_name: String,
    pub environment: String,
    pub tenant_id: Option<String>,
    pub dry_run: bool,
    pub log_level: String,
    pub log_json: bool,
}

impl GeneralConfig {
    pub(crate) fn from_flat(flat: &FlatConfig) -> Self {
        Self {
            service_name: flat
                .service_name
                .clone()
                .unwrap_or_else(|| "relaymail-email-ses".to_string()),
            environment: flat
                .environment
                .clone()
                .unwrap_or_else(|| "local".to_string()),
            tenant_id: flat.tenant_id.clone().filter(|s| !s.is_empty()),
            dry_run: flat.dry_run.unwrap_or(false),
            log_level: flat.log_level.clone().unwrap_or_else(|| "info".to_string()),
            log_json: flat.log_json.unwrap_or(true),
        }
    }
}
