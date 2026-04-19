use relaymail_core::object_store::FetchedObject;
use relaymail_email::{validate, EmailMetadata, MaxSize, RawEmail};

use super::error::StageError;

pub(crate) fn to_raw_and_metadata(
    fetched: FetchedObject,
    max_bytes: u64,
) -> Result<(RawEmail, EmailMetadata), StageError> {
    let raw = RawEmail::new(fetched.bytes);
    let headers = validate(&raw, MaxSize::new(max_bytes))?;
    let size = raw.len() as u64;
    let meta = EmailMetadata::from_headers(&headers, size);
    Ok((raw, meta))
}
