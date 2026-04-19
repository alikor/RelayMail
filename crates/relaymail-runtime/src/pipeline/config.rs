/// What to do with a source object after a successful send.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SuccessDispositionMode {
    Tag,
    Move,
    Delete,
    None,
}

/// What to do with a source object after a permanent failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureDispositionMode {
    Tag,
    Move,
    None,
}

/// Configuration consumed by the pipeline stages.
#[derive(Clone, Debug)]
pub struct ProcessingConfig {
    pub service_name: String,
    pub provider_label: String,
    pub bucket_allowlist: Vec<String>,
    pub prefix_allowlist: Vec<String>,
    pub supported_extensions: Vec<String>,
    pub max_object_size_bytes: u64,
    pub success_mode: SuccessDispositionMode,
    pub failure_mode: FailureDispositionMode,
    pub success_prefix: String,
    pub failure_prefix: String,
    pub delete_unsupported_messages: bool,
    pub delete_invalid_email_messages: bool,
    pub dry_run: bool,
    pub idempotency_ttl_seconds: u64,
}

impl ProcessingConfig {
    pub fn matches_extension(&self, key: &str) -> bool {
        let lower = key.to_ascii_lowercase();
        self.supported_extensions
            .iter()
            .any(|e| lower.ends_with(&e.to_ascii_lowercase()))
    }

    pub fn matches_bucket(&self, bucket: &str) -> bool {
        self.bucket_allowlist.is_empty() || self.bucket_allowlist.iter().any(|b| b == bucket)
    }

    pub fn matches_prefix(&self, key: &str) -> bool {
        self.prefix_allowlist.is_empty() || self.prefix_allowlist.iter().any(|p| key.starts_with(p))
    }
}
