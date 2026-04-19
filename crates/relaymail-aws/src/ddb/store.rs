use aws_sdk_dynamodb::Client;

/// Runtime config for the DynamoDB idempotency store.
#[derive(Clone, Debug)]
pub struct DynamoIdempotencyStoreConfig {
    pub table_name: String,
    pub pk_attribute: String,
    pub ttl_attribute: String,
}

impl DynamoIdempotencyStoreConfig {
    pub fn new(table_name: impl Into<String>) -> Self {
        Self {
            table_name: table_name.into(),
            pk_attribute: "idempotency_key".to_string(),
            ttl_attribute: "ttl".to_string(),
        }
    }
}

/// DynamoDB-backed idempotency store.
#[derive(Clone, Debug)]
pub struct DynamoIdempotencyStore {
    client: Client,
    config: DynamoIdempotencyStoreConfig,
}

impl DynamoIdempotencyStore {
    pub fn new(client: Client, config: DynamoIdempotencyStoreConfig) -> Self {
        Self { client, config }
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }

    pub(crate) fn config(&self) -> &DynamoIdempotencyStoreConfig {
        &self.config
    }
}
