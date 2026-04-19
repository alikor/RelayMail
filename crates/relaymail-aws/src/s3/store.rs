use aws_sdk_s3::Client;

/// Adapter holding an AWS S3 client.
#[derive(Clone, Debug)]
pub struct S3ObjectStore {
    client: Client,
}

impl S3ObjectStore {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }
}
