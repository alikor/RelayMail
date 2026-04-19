/// Percent-decode an S3 object key. S3 event keys arrive URL-encoded with
/// `+` representing spaces per the old S3 notification spec.
pub(crate) fn url_decode_key(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut out = String::with_capacity(raw.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'+' {
            out.push(' ');
            i += 1;
        } else if b == b'%' && i + 2 < bytes.len() {
            let hi = hex_digit(bytes[i + 1]);
            let lo = hex_digit(bytes[i + 2]);
            if let (Some(h), Some(l)) = (hi, lo) {
                out.push(((h << 4) | l) as char);
                i += 3;
                continue;
            }
            out.push(b as char);
            i += 1;
        } else {
            out.push(b as char);
            i += 1;
        }
    }
    out
}

fn hex_digit(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_common_sequences() {
        assert_eq!(url_decode_key("foo%2Fbar"), "foo/bar");
        assert_eq!(url_decode_key("a+b"), "a b");
        assert_eq!(url_decode_key("plain"), "plain");
    }
}
