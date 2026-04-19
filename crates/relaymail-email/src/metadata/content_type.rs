/// Narrow enum over the content-type headers we care about.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContentType {
    TextPlain,
    TextHtml,
    MultipartMixed,
    MultipartAlternative,
    MultipartRelated,
    Other(String),
}

impl ContentType {
    pub fn from_header(raw: Option<&str>) -> Self {
        let raw = match raw {
            Some(v) => v.trim().to_ascii_lowercase(),
            None => return Self::TextPlain,
        };
        let base = raw.split(';').next().unwrap_or("").trim();
        match base {
            "text/plain" => Self::TextPlain,
            "text/html" => Self::TextHtml,
            "multipart/mixed" => Self::MultipartMixed,
            "multipart/alternative" => Self::MultipartAlternative,
            "multipart/related" => Self::MultipartRelated,
            other => Self::Other(other.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_common_types() {
        assert_eq!(
            ContentType::from_header(Some("text/plain; charset=UTF-8")),
            ContentType::TextPlain
        );
        assert_eq!(
            ContentType::from_header(Some("multipart/mixed; boundary=x")),
            ContentType::MultipartMixed
        );
        assert_eq!(ContentType::from_header(None), ContentType::TextPlain);
        assert!(matches!(
            ContentType::from_header(Some("application/pdf")),
            ContentType::Other(_)
        ));
    }
}
