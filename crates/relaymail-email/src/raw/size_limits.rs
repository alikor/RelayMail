use super::message::RawEmail;
use crate::error::EmailError;

/// Upper bound on raw email size, in bytes.
///
/// SES caps raw messages around 40MB (base64-expanded). Default is 10MB to
/// match common inbound configurations; override via configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MaxSize(u64);

impl MaxSize {
    pub const DEFAULT_BYTES: u64 = 10 * 1024 * 1024;

    pub fn new(bytes: u64) -> Self {
        Self(bytes)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn enforce(self, raw: &RawEmail) -> Result<(), EmailError> {
        let actual = raw.len() as u64;
        if actual > self.0 {
            return Err(EmailError::TooLarge {
                actual,
                limit: self.0,
            });
        }
        Ok(())
    }
}

impl Default for MaxSize {
    fn default() -> Self {
        Self(Self::DEFAULT_BYTES)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_under_limit_and_rejects_over() {
        let limit = MaxSize::new(4);
        assert!(limit.enforce(&RawEmail::from_slice(b"1234")).is_ok());
        assert!(matches!(
            limit.enforce(&RawEmail::from_slice(b"12345")),
            Err(EmailError::TooLarge {
                actual: 5,
                limit: 4
            })
        ));
    }
}
