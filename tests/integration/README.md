# Integration tests

Integration against real AWS is not part of the default CI build. The
workspace compiles and exercises its pipeline end-to-end against fake
adapters in `relaymail-testing`.

## LocalStack

Most of the pipeline can be exercised against [LocalStack](https://localstack.cloud):

```sh
docker run --rm -p 4566:4566 localstack/localstack:latest

RELAYMAIL_AWS_ENDPOINT_URL=http://localhost:4566 \
RELAYMAIL_AWS_REGION=us-east-1 \
RELAYMAIL_SQS_QUEUE_URL=http://localhost:4566/000000000000/relaymail \
RELAYMAIL_S3_BUCKET_ALLOWLIST=relaymail-inbound \
RELAYMAIL_DRY_RUN=true \
cargo run --bin relaymail-email-ses
```

Use `DRY_RUN=true` for local loops — LocalStack's SES v2 support is
patchy and not worth wiring into CI.

## Feature-flagged live tests (future)

When full integration tests are added, gate them with a feature flag
plus `#[ignore]` so they only run with `cargo test --features aws-live
-- --ignored`. Never let them gate the default test suite.
