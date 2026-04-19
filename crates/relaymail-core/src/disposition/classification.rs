/// Why something failed. Drives retry vs. terminal decisions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorClassification {
    /// Retryable — the worker should back off and try again.
    Transient,

    /// Permanent failure caused by the sender-side configuration
    /// (e.g. SES identity not verified, IAM permission denied).
    PermanentSender,

    /// Permanent failure caused by the recipient (e.g. hard bounce).
    PermanentRecipient,

    /// The input itself is invalid; retrying will not help.
    Validation,

    /// Unknown / unclassified — treated as permanent to avoid retry storms.
    Unknown,
}

impl ErrorClassification {
    pub fn is_transient(self) -> bool {
        matches!(self, Self::Transient)
    }

    pub fn is_permanent(self) -> bool {
        !self.is_transient()
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Transient => "transient",
            Self::PermanentSender => "permanent_sender",
            Self::PermanentRecipient => "permanent_recipient",
            Self::Validation => "validation",
            Self::Unknown => "unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transient_vs_permanent() {
        assert!(ErrorClassification::Transient.is_transient());
        assert!(ErrorClassification::Validation.is_permanent());
        assert_eq!(ErrorClassification::Transient.label(), "transient");
    }
}
