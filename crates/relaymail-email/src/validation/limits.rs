use crate::error::EmailError;
use crate::parse::ParsedHeaders;

/// SES v2 accepts up to 50 recipients per raw send call. Keep a conservative
/// bound in the domain layer; callers may tighten further via configuration.
const MAX_RECIPIENTS: usize = 50;

pub(crate) fn check(headers: &ParsedHeaders) -> Result<(), EmailError> {
    if headers.recipient_count() > MAX_RECIPIENTS {
        return Err(EmailError::InvalidHeader {
            name: "To/Cc/Bcc",
            reason: format!(
                "too many recipients ({} > {MAX_RECIPIENTS})",
                headers.recipient_count()
            ),
        });
    }
    Ok(())
}
