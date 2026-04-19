# RelayMail

**RelayMail** is a shared outbound-messaging capability. Its long-term purpose
is to reliably dispatch prepared outbound messages through different delivery
providers and delivery methods.

This repository is a Rust Cargo workspace that ships with one production
service today — **`relaymail-email-ses`** — and is designed so future
services (direct SMTP/MTA sender, submission API, delivery-event pipeline,
operator console) can plug into the same shared crates without disturbing
the domain core.

## What ships today

| Piece | Status | Purpose |
|---|---|---|
| `crates/relaymail-core` | Implemented | Domain types + capability traits |
| `crates/relaymail-email` | Implemented | Raw-MIME wrapper, validation, redaction |
| `crates/relaymail-delivery` | Implemented | `EmailSender` provider trait + result/error |
| `crates/relaymail-aws` | Implemented | AWS adapter impls (S3, SQS, SES v2, DynamoDB) |
| `crates/relaymail-runtime` | Implemented | HTTP health/metrics, tracing, worker, pipeline |
| `crates/relaymail-testing` | Implemented | Fakes + fixtures used by workspace tests |
| `apps/relaymail-email-ses` | Implemented | Worker binary: S3 → SQS → SES v2 |
| `apps/relaymail-direct-mta` | Placeholder | Future SMTP/MTA sender |
| `apps/relaymail-api` | Placeholder | Future submission API |
| `apps/relaymail-events` | Placeholder | Future bounce/complaint/delivery pipeline |
| `apps/relaymail-console` | Placeholder | Future operator console |

## Architecture in one sentence

`relaymail-email-ses` long-polls SQS for S3 `ObjectCreated` events,
downloads each object from S3, validates it as a raw RFC-5322 message,
calls SES v2 `SendEmail` with the raw MIME, records the send in a
DynamoDB-backed idempotency table (or an in-memory store for dev), tags
the object with the outcome, then acks the SQS message.

See [docs/architecture.md](docs/architecture.md).

## Build & test

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Coverage (optional locally, required in CI):

```sh
cargo llvm-cov --workspace --all-features --fail-under-lines 80
```

File-length gate (no handwritten `.rs` over 100 physical lines):

```sh
bash scripts/check-file-length.sh
```

## Run locally (dry-run)

```sh
RELAYMAIL_DRY_RUN=true \
RELAYMAIL_AWS_REGION=us-east-1 \
RELAYMAIL_SQS_QUEUE_URL=http://localhost/stub \
RELAYMAIL_S3_BUCKET_ALLOWLIST=test-bucket \
cargo run --bin relaymail-email-ses
```

Dry-run mode fetches + validates but never calls SES and never touches
the source object — useful when pointed at LocalStack.

## Deploy

- Dockerfile: [deploy/docker/Dockerfile.relaymail-email-ses](deploy/docker/Dockerfile.relaymail-email-ses)
  (multi-stage, distroless, non-root).
- Kubernetes manifests: [deploy/k8s/relaymail-email-ses](deploy/k8s/relaymail-email-ses/)
  (kustomize base with probes, PDB, HPA, IRSA placeholder).
- Helm chart skeleton: [deploy/helm/relaymail-email-ses](deploy/helm/relaymail-email-ses/).
- Terraform example: [deploy/terraform/aws-relaymail-email-ses-example](deploy/terraform/aws-relaymail-email-ses-example/).

## Layout

```
crates/    — reusable libraries
apps/      — binaries (today: one real, four placeholders)
deploy/    — docker, k8s, helm, terraform
docs/      — architecture & ops
examples/  — .eml + event fixtures, config examples
```

## Naming conventions

See [docs/naming.md](docs/naming.md). Short version:

- Capability: `RelayMail`
- First service: `RelayMail SES` / binary `relaymail-email-ses`
- Future direct MTA: `relaymail-direct-mta`
- Env-var prefix: `RELAYMAIL_`
- Metrics namespace: `relaymail_*`
- S3 tag keys: `relaymail-*`

## House rules

Every change follows `docs/practices/`: ≤100 physical lines per `.rs`
file, ≥80% line coverage, narrow visibility, no adapter types in the
domain crates, `thiserror` for library errors.
