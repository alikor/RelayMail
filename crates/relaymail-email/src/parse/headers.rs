use super::addresses::Mailbox;

/// Minimal parsed headers needed for validation and metadata.
///
/// `configuration_set` is the SES configuration set name the sender has
/// asked us to use for this specific message, via the
/// `X-SES-CONFIGURATION-SET` header. Mirrored by SES's own documented
/// header name — when present, it overrides the static default in
/// `SesRuntimeConfig.configuration_set` per-email, which is how
/// producers (e.g. JobVia backend-rs, identity-rs) opt individual
/// messages into tracked delivery.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ParsedHeaders {
    pub(crate) from: Vec<Mailbox>,
    pub(crate) to: Vec<Mailbox>,
    pub(crate) cc: Vec<Mailbox>,
    pub(crate) bcc: Vec<Mailbox>,
    pub(crate) subject: Option<String>,
    pub(crate) message_id: Option<String>,
    pub(crate) date: Option<String>,
    pub(crate) content_type: Option<String>,
    pub(crate) configuration_set: Option<String>,
}

impl ParsedHeaders {
    pub fn from(&self) -> &[Mailbox] {
        &self.from
    }

    pub fn to(&self) -> &[Mailbox] {
        &self.to
    }

    pub fn cc(&self) -> &[Mailbox] {
        &self.cc
    }

    pub fn bcc(&self) -> &[Mailbox] {
        &self.bcc
    }

    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    pub fn message_id(&self) -> Option<&str> {
        self.message_id.as_deref()
    }

    pub fn date(&self) -> Option<&str> {
        self.date.as_deref()
    }

    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// Value of the `X-SES-CONFIGURATION-SET` header, if set by the
    /// producer. Takes precedence over the static runtime config.
    pub fn configuration_set(&self) -> Option<&str> {
        self.configuration_set.as_deref()
    }

    pub fn recipient_count(&self) -> usize {
        self.to.len() + self.cc.len() + self.bcc.len()
    }
}
