use super::flat::FlatConfig;

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub(crate) struct SesConfig {
    pub region: Option<String>,
    pub configuration_set: Option<String>,
    pub source_arn: Option<String>,
    pub from_arn: Option<String>,
    pub return_path_arn: Option<String>,
}

impl SesConfig {
    pub(crate) fn from_flat(flat: &FlatConfig) -> Self {
        Self {
            region: flat.ses_region.clone().filter(|s| !s.is_empty()),
            configuration_set: flat.ses_configuration_set.clone().filter(|s| !s.is_empty()),
            source_arn: flat.ses_source_arn.clone().filter(|s| !s.is_empty()),
            from_arn: flat.ses_from_arn.clone().filter(|s| !s.is_empty()),
            return_path_arn: flat.ses_return_path_arn.clone().filter(|s| !s.is_empty()),
        }
    }
}
