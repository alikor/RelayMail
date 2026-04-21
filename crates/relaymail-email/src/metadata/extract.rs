use super::content_type::ContentType;
use crate::parse::{Mailbox, ParsedHeaders};

/// Domain-level metadata for a parsed email.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmailMetadata {
    from: Vec<Mailbox>,
    recipients: Vec<Mailbox>,
    subject: Option<String>,
    message_id: Option<String>,
    content_type: ContentType,
    size_bytes: u64,
    configuration_set: Option<String>,
}

impl EmailMetadata {
    pub fn from_headers(headers: &ParsedHeaders, size_bytes: u64) -> Self {
        let mut recipients = Vec::new();
        recipients.extend_from_slice(headers.to());
        recipients.extend_from_slice(headers.cc());
        recipients.extend_from_slice(headers.bcc());
        Self {
            from: headers.from().to_vec(),
            recipients,
            subject: headers.subject().map(str::to_string),
            message_id: headers.message_id().map(str::to_string),
            content_type: ContentType::from_header(headers.content_type()),
            size_bytes,
            configuration_set: headers.configuration_set().map(str::to_string),
        }
    }

    pub fn senders(&self) -> &[Mailbox] {
        &self.from
    }

    pub fn recipients(&self) -> &[Mailbox] {
        &self.recipients
    }

    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    pub fn message_id(&self) -> Option<&str> {
        self.message_id.as_deref()
    }

    pub fn content_type(&self) -> &ContentType {
        &self.content_type
    }

    pub fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    /// Per-message SES configuration set, extracted from the
    /// `X-SES-CONFIGURATION-SET` header. When `Some`, the SES adapter
    /// uses this value in place of the runtime default so producers
    /// can route individual messages to different event destinations.
    pub fn configuration_set(&self) -> Option<&str> {
        self.configuration_set.as_deref()
    }
}
