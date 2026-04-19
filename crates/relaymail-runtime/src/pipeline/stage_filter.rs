use super::config::ProcessingConfig;
use super::event_parser::ObjectRef;
use super::outcome::PipelineOutcome;

/// Result of filtering a single object ref against bucket/prefix/extension
/// allowlists.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum FilterDecision {
    Pass,
    Skip(PipelineOutcome),
}

pub(crate) fn filter(config: &ProcessingConfig, object: &ObjectRef) -> FilterDecision {
    if !config.matches_bucket(object.object.bucket()) {
        return FilterDecision::Skip(PipelineOutcome::SkippedUnsupportedBucket);
    }
    if !config.matches_prefix(object.object.key()) {
        return FilterDecision::Skip(PipelineOutcome::SkippedUnsupportedPrefix);
    }
    if !config.matches_extension(object.object.key()) {
        return FilterDecision::Skip(PipelineOutcome::SkippedUnsupportedExtension);
    }
    FilterDecision::Pass
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use relaymail_core::ObjectId;

    fn cfg() -> ProcessingConfig {
        ProcessingConfig {
            service_name: "t".into(),
            provider_label: "t".into(),
            bucket_allowlist: vec!["ok-bucket".into()],
            prefix_allowlist: vec!["incoming/".into()],
            supported_extensions: vec![".eml".into()],
            max_object_size_bytes: 1024,
            success_mode: super::super::config::SuccessDispositionMode::Tag,
            failure_mode: super::super::config::FailureDispositionMode::Tag,
            success_prefix: "processed/".into(),
            failure_prefix: "failed/".into(),
            delete_unsupported_messages: true,
            delete_invalid_email_messages: true,
            dry_run: false,
            idempotency_ttl_seconds: 60,
        }
    }

    #[test]
    fn filters_bucket_prefix_extension() {
        let c = cfg();
        let obj = ObjectRef {
            object: ObjectId::new("other", "incoming/a.eml"),
            etag: "e".into(),
            size: 1,
            event_time: Utc::now(),
        };
        assert_eq!(
            filter(&c, &obj),
            FilterDecision::Skip(PipelineOutcome::SkippedUnsupportedBucket)
        );
    }
}
