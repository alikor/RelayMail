/// Outcome of processing one envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PipelineOutcome {
    Sent {
        provider_message_id: String,
    },
    DryRunSent,
    SkippedAlreadyClaimed,
    SkippedUnsupportedExtension,
    SkippedUnsupportedBucket,
    SkippedUnsupportedPrefix,
    UnknownEnvelope,
    Failed {
        classification_label: &'static str,
        reason: String,
    },
    Retry {
        reason: String,
    },
}
