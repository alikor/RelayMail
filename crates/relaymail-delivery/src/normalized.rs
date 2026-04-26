use std::collections::BTreeMap;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::{DateTime, Utc};
use mailparse::{DispositionType, MailHeaderMap, ParsedMail};
use relaymail_core::{IdempotencyKey, MessageId, TenantId};
use relaymail_email::{parse_headers_only, Mailbox, RawEmail};
use serde::{Deserialize, Serialize};

use crate::SendError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EmailAddress {
    pub address: String,
    pub display_name: Option<String>,
}

impl EmailAddress {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            display_name: None,
        }
    }

    pub fn from_mailbox(mailbox: &Mailbox) -> Self {
        Self {
            address: mailbox.address().to_string(),
            display_name: mailbox.display_name().map(str::to_string),
        }
    }

    pub fn domain(&self) -> Option<&str> {
        self.address.rsplit_once('@').map(|(_, domain)| domain)
    }

    pub fn to_header_value(&self) -> String {
        match self.display_name.as_deref().filter(|s| !s.is_empty()) {
            Some(name) => format!("{} <{}>", quote_display_name(name), self.address),
            None => self.address.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub filename: String,
    pub content_type: String,
    pub content_base64: String,
    pub content_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EmailSendStatus {
    Accepted,
    Rejected,
    Failed,
    Suppressed,
    ValidationFailed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EmailSendRequest {
    pub internal_message_id: MessageId,
    pub correlation_id: Option<String>,
    pub stream: String,
    pub category: Option<String>,
    pub from: Option<EmailAddress>,
    pub reply_to: Option<EmailAddress>,
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub subject: Option<String>,
    pub html_body: Option<String>,
    pub text_body: Option<String>,
    pub template_key: Option<String>,
    pub template_data: BTreeMap<String, serde_json::Value>,
    pub attachments: Vec<EmailAttachment>,
    pub custom_headers: BTreeMap<String, String>,
    pub metadata: BTreeMap<String, String>,
    pub consent_metadata: BTreeMap<String, String>,
    pub unsubscribe_url: Option<String>,
    pub idempotency_key: String,
    pub tenant: Option<TenantId>,
}

impl EmailSendRequest {
    pub fn from_raw(
        raw: &RawEmail,
        idempotency_key: &IdempotencyKey,
        tenant: Option<TenantId>,
    ) -> Result<Self, SendError> {
        let headers = parse_headers_only(raw.as_bytes())
            .map_err(|e| SendError::Validation(format!("parse headers: {e}")))?;
        let parsed = mailparse::parse_mail(raw.as_bytes())
            .map_err(|e| SendError::Validation(format!("parse MIME: {e}")))?;
        let mut bodies = BodyParts::default();
        collect_parts(&parsed, &mut bodies)?;
        let header_map = &parsed.headers;
        let stream = header_value(header_map, "X-RelayMail-Stream")
            .unwrap_or_else(|| "transactional".to_string())
            .trim()
            .to_ascii_lowercase();
        let mut metadata = relaymail_headers(header_map, "X-RelayMail-Metadata-");
        metadata.insert("raw_message_size_bytes".into(), raw.len().to_string());
        if let Some(message_id) = headers.message_id() {
            metadata.insert("source_message_id".into(), message_id.to_string());
        }
        Ok(Self {
            internal_message_id: MessageId::new(),
            correlation_id: header_value(header_map, "X-RelayMail-Correlation-Id"),
            stream,
            category: header_value(header_map, "X-RelayMail-Category"),
            from: headers.from().first().map(EmailAddress::from_mailbox),
            reply_to: header_value(header_map, "Reply-To")
                .and_then(|v| mailparse::addrparse(&v).ok())
                .and_then(|list| list.iter().next().cloned())
                .and_then(mail_addr_to_email),
            to: headers
                .to()
                .iter()
                .map(EmailAddress::from_mailbox)
                .collect(),
            cc: headers
                .cc()
                .iter()
                .map(EmailAddress::from_mailbox)
                .collect(),
            bcc: headers
                .bcc()
                .iter()
                .map(EmailAddress::from_mailbox)
                .collect(),
            subject: headers.subject().map(str::to_string),
            html_body: bodies.html,
            text_body: bodies.text,
            template_key: header_value(header_map, "X-RelayMail-Template-Key"),
            template_data: BTreeMap::new(),
            attachments: bodies.attachments,
            custom_headers: safe_custom_headers(header_map),
            metadata,
            consent_metadata: relaymail_headers(header_map, "X-RelayMail-Consent-"),
            unsubscribe_url: header_value(header_map, "X-RelayMail-Unsubscribe-Url")
                .or_else(|| header_value(header_map, "List-Unsubscribe").map(clean_unsubscribe)),
            idempotency_key: idempotency_key.as_str().to_string(),
            tenant,
        })
    }

    pub fn recipients(&self) -> impl Iterator<Item = &EmailAddress> {
        self.to.iter().chain(self.cc.iter()).chain(self.bcc.iter())
    }

    pub fn recipient_count(&self) -> usize {
        self.recipients().count()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EmailSendResult {
    pub internal_message_id: MessageId,
    pub provider: String,
    pub provider_message_id: String,
    pub accepted_at_utc: Option<DateTime<Utc>>,
    pub status: EmailSendStatus,
    pub attempt_count: u32,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub raw_provider_response: Option<String>,
    pub correlation_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EmailEventType {
    Sent,
    Accepted,
    Delivered,
    DeliveryDelayed,
    PermanentBounce,
    TransientBounce,
    Complaint,
    Rejected,
    Failed,
    Suppressed,
    Opened,
    Clicked,
    Unsubscribed,
    Unknown,
}

#[derive(Default)]
struct BodyParts {
    html: Option<String>,
    text: Option<String>,
    attachments: Vec<EmailAttachment>,
}

fn collect_parts(part: &ParsedMail<'_>, out: &mut BodyParts) -> Result<(), SendError> {
    if part.subparts.is_empty() {
        collect_leaf(part, out)?;
    } else {
        for child in &part.subparts {
            collect_parts(child, out)?;
        }
    }
    Ok(())
}

fn collect_leaf(part: &ParsedMail<'_>, out: &mut BodyParts) -> Result<(), SendError> {
    let disposition = part.get_content_disposition();
    let content_type = part.ctype.mimetype.to_ascii_lowercase();
    let filename = disposition
        .params
        .get("filename")
        .or_else(|| part.ctype.params.get("name"))
        .cloned();
    let is_attachment =
        matches!(disposition.disposition, DispositionType::Attachment) || filename.is_some();
    if is_attachment {
        let bytes = part
            .get_body_raw()
            .map_err(|e| SendError::Validation(format!("decode attachment: {e}")))?;
        out.attachments.push(EmailAttachment {
            filename: filename.unwrap_or_else(|| "attachment".to_string()),
            content_type,
            content_base64: STANDARD.encode(bytes),
            content_id: part.headers.get_first_value("Content-ID"),
        });
        return Ok(());
    }
    if content_type == "text/html" && out.html.is_none() {
        out.html = Some(
            part.get_body()
                .map_err(|e| SendError::Validation(format!("decode html body: {e}")))?,
        );
    } else if content_type == "text/plain" && out.text.is_none() {
        out.text = Some(
            part.get_body()
                .map_err(|e| SendError::Validation(format!("decode text body: {e}")))?,
        );
    }
    Ok(())
}

fn mail_addr_to_email(addr: mailparse::MailAddr) -> Option<EmailAddress> {
    match addr {
        mailparse::MailAddr::Single(info) => Some(EmailAddress {
            address: info.addr,
            display_name: info.display_name,
        }),
        mailparse::MailAddr::Group(group) => {
            group.addrs.into_iter().next().map(|info| EmailAddress {
                address: info.addr,
                display_name: info.display_name,
            })
        }
    }
}

fn header_value(headers: &[mailparse::MailHeader<'_>], name: &str) -> Option<String> {
    headers
        .get_first_value(name)
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn safe_custom_headers(headers: &[mailparse::MailHeader<'_>]) -> BTreeMap<String, String> {
    headers
        .iter()
        .filter_map(|h| {
            let key = h.get_key();
            let value = h.get_value().trim().to_string();
            if value.is_empty() || !is_safe_header_name(&key) || is_reserved_header(&key) {
                None
            } else {
                Some((key, value))
            }
        })
        .collect()
}

fn relaymail_headers(
    headers: &[mailparse::MailHeader<'_>],
    prefix: &str,
) -> BTreeMap<String, String> {
    headers
        .iter()
        .filter_map(|h| {
            let key = h.get_key();
            key.strip_prefix(prefix).and_then(|name| {
                let value = h.get_value().trim().to_string();
                if name.is_empty() || value.is_empty() {
                    None
                } else {
                    Some((name.to_ascii_lowercase(), value))
                }
            })
        })
        .collect()
}

fn is_safe_header_name(name: &str) -> bool {
    name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') && !name.is_empty()
}

fn is_reserved_header(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "from"
            | "to"
            | "cc"
            | "bcc"
            | "subject"
            | "date"
            | "message-id"
            | "mime-version"
            | "content-type"
            | "content-disposition"
            | "content-transfer-encoding"
            | "return-path"
            | "sender"
    )
}

fn clean_unsubscribe(value: String) -> String {
    value
        .trim()
        .trim_start_matches('<')
        .trim_end_matches('>')
        .to_string()
}

fn quote_display_name(name: &str) -> String {
    if name
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b' ' | b'.' | b'-' | b'_'))
    {
        name.to_string()
    } else {
        format!("\"{}\"", name.replace('"', "\\\""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use relaymail_core::{IdempotencyKey, ObjectId};

    fn key() -> IdempotencyKey {
        IdempotencyKey::compute(None, &ObjectId::new("b", "k.eml"), "e", 1)
    }

    #[test]
    fn normalizes_basic_raw_email() {
        let raw = RawEmail::from_slice(
            b"From: Example <no-reply@mail.example.com>\r\n\
              To: user@example.net\r\n\
              Subject: Hello\r\n\
              X-RelayMail-Stream: marketing\r\n\
              X-RelayMail-Consent-Source: signup\r\n\
              X-RelayMail-Unsubscribe-Url: https://example.com/u/1\r\n\
              X-Custom: yes\r\n\
              \r\n\
              Body",
        );
        let req = EmailSendRequest::from_raw(&raw, &key(), None).unwrap();
        assert_eq!(req.stream, "marketing");
        assert_eq!(req.from.unwrap().domain(), Some("mail.example.com"));
        assert_eq!(req.to[0].address, "user@example.net");
        assert_eq!(req.text_body.as_deref(), Some("Body"));
        assert_eq!(
            req.custom_headers.get("X-Custom").map(String::as_str),
            Some("yes")
        );
        assert_eq!(
            req.consent_metadata.get("source").map(String::as_str),
            Some("signup")
        );
    }

    #[test]
    fn reserved_headers_are_not_custom_headers() {
        let raw = RawEmail::from_slice(
            b"From: a@mail.example.com\r\nTo: b@example.net\r\nSubject: Hi\r\n\r\nBody",
        );
        let req = EmailSendRequest::from_raw(&raw, &key(), None).unwrap();
        assert!(!req.custom_headers.contains_key("From"));
        assert!(!req.custom_headers.contains_key("Subject"));
    }
}
