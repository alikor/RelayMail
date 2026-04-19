use std::path::PathBuf;

use super::aws::AwsConfig;
use super::error::AppConfigError;
use super::general::GeneralConfig;
use super::polling::PollingConfig;
use super::processing::ProcessingConfig;
use super::runtime::RuntimeConfig;
use super::s3_filter::S3FilterConfig;
use super::ses::SesConfig;
use super::sources;
use super::sqs::SqsConfig;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct AppConfig {
    pub general: GeneralConfig,
    pub aws: AwsConfig,
    pub ses: SesConfig,
    pub s3_filter: S3FilterConfig,
    pub sqs: SqsConfig,
    pub processing: ProcessingConfig,
    pub runtime: RuntimeConfig,
    pub polling: PollingConfig,
}

pub(crate) fn load(config_file: Option<PathBuf>) -> Result<AppConfig, AppConfigError> {
    let figment = match &config_file {
        Some(path) => sources::with_yaml(path),
        None => sources::build(),
    };
    let flat = sources::extract(figment)?;
    Ok(AppConfig {
        general: GeneralConfig::from_flat(&flat),
        aws: AwsConfig::from_flat(&flat),
        ses: SesConfig::from_flat(&flat),
        s3_filter: S3FilterConfig::from_flat(&flat),
        sqs: SqsConfig::from_flat(&flat)?,
        processing: ProcessingConfig::from_flat(&flat)?,
        runtime: RuntimeConfig::from_flat(&flat)?,
        polling: PollingConfig::from_flat(&flat),
    })
}
