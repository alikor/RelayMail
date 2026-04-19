use super::flat::FlatConfig;

#[derive(Clone, Debug)]
pub(crate) struct AwsConfig {
    pub region: Option<String>,
    pub endpoint_url: Option<String>,
}

impl AwsConfig {
    pub(crate) fn from_flat(flat: &FlatConfig) -> Self {
        Self {
            region: flat.aws_region.clone().filter(|s| !s.is_empty()),
            endpoint_url: flat.aws_endpoint_url.clone().filter(|s| !s.is_empty()),
        }
    }
}
