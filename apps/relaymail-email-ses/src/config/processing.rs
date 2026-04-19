use relaymail_runtime::pipeline::{FailureDispositionMode, SuccessDispositionMode};

use super::error::AppConfigError;
use super::flat::FlatConfig;

#[derive(Clone, Debug)]
pub(crate) struct ProcessingConfig {
    pub success_mode: SuccessDispositionMode,
    pub failure_mode: FailureDispositionMode,
    pub success_prefix: String,
    pub failure_prefix: String,
    pub delete_unsupported_messages: bool,
    pub delete_invalid_email_messages: bool,
}

impl ProcessingConfig {
    pub(crate) fn from_flat(flat: &FlatConfig) -> Result<Self, AppConfigError> {
        let success = match flat.processing_success_mode.as_deref().unwrap_or("tag") {
            "tag" => SuccessDispositionMode::Tag,
            "move" => SuccessDispositionMode::Move,
            "delete" => SuccessDispositionMode::Delete,
            "none" => SuccessDispositionMode::None,
            other => return Err(AppConfigError::Invalid(format!("success_mode: {other}"))),
        };
        let failure = match flat.processing_failure_mode.as_deref().unwrap_or("tag") {
            "tag" => FailureDispositionMode::Tag,
            "move" => FailureDispositionMode::Move,
            "none" => FailureDispositionMode::None,
            other => return Err(AppConfigError::Invalid(format!("failure_mode: {other}"))),
        };
        Ok(Self {
            success_mode: success,
            failure_mode: failure,
            success_prefix: flat
                .success_prefix
                .clone()
                .unwrap_or_else(|| "processed/".into()),
            failure_prefix: flat
                .failure_prefix
                .clone()
                .unwrap_or_else(|| "failed/".into()),
            delete_unsupported_messages: flat.delete_unsupported_messages.unwrap_or(true),
            delete_invalid_email_messages: flat.delete_invalid_email_messages.unwrap_or(true),
        })
    }
}
