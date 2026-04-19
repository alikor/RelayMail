use std::path::Path;

use figment::providers::{Env, Format, Yaml};
use figment::Figment;

use super::flat::FlatConfig;

pub(crate) fn build() -> Figment {
    Figment::new().merge(Env::prefixed("RELAYMAIL_"))
}

pub(crate) fn with_yaml(path: &Path) -> Figment {
    Figment::new()
        .merge(Yaml::file(path))
        .merge(Env::prefixed("RELAYMAIL_"))
}

pub(crate) fn extract(figment: Figment) -> Result<FlatConfig, Box<figment::Error>> {
    figment.extract().map_err(Box::new)
}
