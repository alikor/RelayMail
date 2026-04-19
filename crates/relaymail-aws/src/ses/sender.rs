use aws_sdk_sesv2::Client;

/// Runtime tuning for the SES sender (loaded from config at boot).
#[derive(Clone, Debug, Default)]
pub struct SesRuntimeConfig {
    pub configuration_set: Option<String>,
    pub source_arn: Option<String>,
    pub from_arn: Option<String>,
    pub return_path_arn: Option<String>,
}

/// SES v2 sender implementing [`relaymail_delivery::EmailSender`].
#[derive(Clone, Debug)]
pub struct SesSender {
    client: Client,
    config: SesRuntimeConfig,
}

impl SesSender {
    pub fn new(client: Client, config: SesRuntimeConfig) -> Self {
        Self { client, config }
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }

    pub(crate) fn runtime_config(&self) -> &SesRuntimeConfig {
        &self.config
    }
}
