pub fn basic_eml() -> &'static [u8] {
    include_bytes!("../../../../examples/eml/basic.eml")
}

pub fn multipart_eml() -> &'static [u8] {
    include_bytes!("../../../../examples/eml/multipart-with-attachment.eml")
}
