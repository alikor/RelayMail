use super::error::AppConfigError;
use super::flat::FlatConfig;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct SqsConfig {
    pub queue_url: String,
    pub max_messages: i32,
    pub wait_time_seconds: i32,
    pub visibility_timeout_seconds: i32,
    pub visibility_extension_enabled: bool,
}

impl SqsConfig {
    pub(crate) fn from_flat(flat: &FlatConfig) -> Result<Self, AppConfigError> {
        let queue_url = flat
            .sqs_queue_url
            .clone()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| AppConfigError::Missing("RELAYMAIL_SQS_QUEUE_URL".to_string()))?;
        Ok(Self {
            queue_url,
            max_messages: flat.sqs_max_messages.unwrap_or(10),
            wait_time_seconds: flat.sqs_wait_time_seconds.unwrap_or(20),
            visibility_timeout_seconds: flat.sqs_visibility_timeout_seconds.unwrap_or(300),
            visibility_extension_enabled: flat.sqs_visibility_extension_enabled.unwrap_or(true),
        })
    }
}
