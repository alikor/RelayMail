use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, SdkConfig};
use aws_sdk_s3::config::Region;

/// Optional knobs for AWS SDK setup.
#[derive(Clone, Debug, Default)]
pub struct AwsConnectOptions {
    pub region: Option<String>,
    pub endpoint_url: Option<String>,
}

/// Load a shared `SdkConfig` from the default credential chain.
///
/// Honors the `RELAYMAIL_AWS_ENDPOINT_URL` override for LocalStack / custom
/// endpoints via `endpoint_url`.
pub async fn load_shared_aws_config(options: AwsConnectOptions) -> SdkConfig {
    let mut loader = aws_config::defaults(BehaviorVersion::latest());
    let region_chain = match options.region {
        Some(r) => RegionProviderChain::first_try(Region::new(r)).or_default_provider(),
        None => RegionProviderChain::default_provider(),
    };
    loader = loader.region(region_chain);
    if let Some(url) = options.endpoint_url {
        loader = loader.endpoint_url(url);
    }
    loader.load().await
}
