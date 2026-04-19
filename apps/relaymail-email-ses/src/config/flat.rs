use serde::Deserialize;

/// Deserialize target for the figment env/YAML composition.
/// All fields are optional so missing values take defaults rather than errors.
#[derive(Debug, Default, Deserialize)]
pub(crate) struct FlatConfig {
    pub service_name: Option<String>,
    pub environment: Option<String>,
    pub tenant_id: Option<String>,
    pub dry_run: Option<bool>,
    pub log_level: Option<String>,
    pub log_json: Option<bool>,

    pub aws_region: Option<String>,
    pub aws_endpoint_url: Option<String>,
    pub ses_region: Option<String>,
    pub sqs_queue_url: Option<String>,
    pub idempotency_table_name: Option<String>,
    pub idempotency_ttl_seconds: Option<u64>,

    pub s3_bucket_allowlist: Option<String>,
    pub s3_prefix_allowlist: Option<String>,
    pub supported_extensions: Option<String>,
    pub max_email_bytes: Option<u64>,

    pub ses_configuration_set: Option<String>,
    pub ses_source_arn: Option<String>,
    pub ses_from_arn: Option<String>,
    pub ses_return_path_arn: Option<String>,

    pub processing_success_mode: Option<String>,
    pub processing_failure_mode: Option<String>,
    pub success_prefix: Option<String>,
    pub failure_prefix: Option<String>,
    pub delete_unsupported_messages: Option<bool>,
    pub delete_invalid_email_messages: Option<bool>,

    pub sqs_max_messages: Option<i32>,
    pub sqs_wait_time_seconds: Option<i32>,
    pub sqs_visibility_timeout_seconds: Option<i32>,
    pub sqs_visibility_extension_enabled: Option<bool>,

    pub worker_concurrency: Option<usize>,
    pub http_bind_addr: Option<String>,
    pub shutdown_grace_period_seconds: Option<u64>,

    pub polling_mode_enabled: Option<bool>,
    pub polling_interval_seconds: Option<u64>,
    pub polling_buckets: Option<String>,
    pub polling_prefixes: Option<String>,
}
