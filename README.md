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

---

## Docker

The image is published to Docker Hub as a multi-arch manifest (`linux/amd64`
and `linux/arm64`).

```sh
docker pull alikor/relaymail:latest
```

Run locally against LocalStack or a real AWS account — all configuration
is through environment variables prefixed `RELAYMAIL_`:

```sh
docker run --rm \
  -e RELAYMAIL_AWS_REGION=us-east-1 \
  -e RELAYMAIL_SQS_QUEUE_URL=https://sqs.us-east-1.amazonaws.com/123456789012/my-queue \
  -e RELAYMAIL_S3_BUCKET_ALLOWLIST=my-email-bucket \
  -e RELAYMAIL_IDEMPOTENCY_TABLE_NAME=relaymail-idempotency \
  -e AWS_ACCESS_KEY_ID=... \
  -e AWS_SECRET_ACCESS_KEY=... \
  -p 8080:8080 \
  alikor/relaymail:latest
```

Health and metrics endpoints are available once the container starts:

```
GET http://localhost:8080/healthz   # liveness
GET http://localhost:8080/readyz    # readiness
GET http://localhost:8080/metrics   # Prometheus exposition
```

### Dry-run mode

Set `RELAYMAIL_DRY_RUN=true` to fetch and validate emails without calling
SES. Useful for testing connectivity and message format before going live.

---

## Docker Compose

For local development with [LocalStack](https://localstack.cloud):

```yaml
# compose.yaml
services:
  relaymail:
    image: alikor/relaymail:latest
    ports:
      - "8080:8080"
    environment:
      RELAYMAIL_AWS_REGION: us-east-1
      RELAYMAIL_AWS_ENDPOINT_URL: http://localstack:4566
      RELAYMAIL_SQS_QUEUE_URL: http://sqs.us-east-1.localhost.localstack.cloud:4566/000000000000/relaymail-queue
      RELAYMAIL_S3_BUCKET_ALLOWLIST: relaymail-emails
      RELAYMAIL_IDEMPOTENCY_TABLE_NAME: relaymail-idempotency
      RELAYMAIL_DRY_RUN: "false"
      RELAYMAIL_LOG_JSON: "false"
      AWS_ACCESS_KEY_ID: test
      AWS_SECRET_ACCESS_KEY: test
    depends_on:
      localstack:
        condition: service_healthy

  localstack:
    image: localstack/localstack:latest
    ports:
      - "4566:4566"
    environment:
      SERVICES: s3,sqs,ses,dynamodb
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:4566/_localstack/health"]
      interval: 5s
      retries: 10
```

```sh
docker compose up
```

---

## Kubernetes

### Plain manifests (Kustomize)

The base manifests are in [`deploy/k8s/relaymail-email-ses/`](deploy/k8s/relaymail-email-ses/).
They include a `Deployment`, `Service`, `HPA`, `PodDisruptionBudget`, and
a `ServiceAccount` with an IRSA annotation placeholder.

1. Copy and edit the config:

```sh
cp deploy/k8s/relaymail-email-ses/configmap.yaml my-overlay/configmap.yaml
# set RELAYMAIL_SQS_QUEUE_URL, RELAYMAIL_S3_BUCKET_ALLOWLIST, etc.
```

2. Apply:

```sh
kubectl apply -k deploy/k8s/relaymail-email-ses/
```

The container image referenced in the deployment can be overridden in your
Kustomize overlay:

```yaml
# my-overlay/kustomization.yaml
resources:
  - ../../deploy/k8s/relaymail-email-ses

images:
  - name: relaymail-email-ses
    newName: alikor/relaymail
    newTag: "sha-a65c0d5"
```

### Helm chart

A Helm chart skeleton is in [`deploy/helm/relaymail-email-ses/`](deploy/helm/relaymail-email-ses/).

**Install:**

```sh
helm install relaymail ./deploy/helm/relaymail-email-ses \
  --namespace relaymail --create-namespace \
  --set image.repository=alikor/relaymail \
  --set image.tag=latest \
  --set env.RELAYMAIL_AWS_REGION=us-east-1 \
  --set env.RELAYMAIL_SQS_QUEUE_URL=https://sqs.us-east-1.amazonaws.com/123456789012/my-queue \
  --set env.RELAYMAIL_S3_BUCKET_ALLOWLIST=my-email-bucket \
  --set env.RELAYMAIL_IDEMPOTENCY_TABLE_NAME=relaymail-idempotency \
  --set serviceAccount.annotations."eks\.amazonaws\.com/role-arn"=arn:aws:iam::123456789012:role/relaymail-irsa
```

**Or use a values file:**

```yaml
# my-values.yaml
image:
  repository: alikor/relaymail
  tag: latest

serviceAccount:
  annotations:
    eks.amazonaws.com/role-arn: arn:aws:iam::123456789012:role/relaymail-irsa

env:
  RELAYMAIL_AWS_REGION: us-east-1
  RELAYMAIL_SQS_QUEUE_URL: https://sqs.us-east-1.amazonaws.com/123456789012/my-queue
  RELAYMAIL_S3_BUCKET_ALLOWLIST: my-email-bucket
  RELAYMAIL_IDEMPOTENCY_TABLE_NAME: relaymail-idempotency
```

```sh
helm install relaymail ./deploy/helm/relaymail-email-ses \
  --namespace relaymail --create-namespace \
  -f my-values.yaml
```

**Upgrade:**

```sh
helm upgrade relaymail ./deploy/helm/relaymail-email-ses \
  --namespace relaymail \
  -f my-values.yaml
```

### AWS IAM (IRSA)

The worker needs the following permissions on its IAM role:

```json
{
  "Statement": [
    {
      "Effect": "Allow",
      "Action": ["s3:GetObject", "s3:GetObjectTagging", "s3:PutObjectTagging",
                 "s3:CopyObject", "s3:DeleteObject"],
      "Resource": "arn:aws:s3:::my-email-bucket/*"
    },
    {
      "Effect": "Allow",
      "Action": ["sqs:ReceiveMessage", "sqs:DeleteMessage",
                 "sqs:ChangeMessageVisibility", "sqs:GetQueueAttributes"],
      "Resource": "arn:aws:sqs:us-east-1:123456789012:my-queue"
    },
    {
      "Effect": "Allow",
      "Action": ["ses:SendEmail", "ses:SendRawEmail"],
      "Resource": "*"
    },
    {
      "Effect": "Allow",
      "Action": ["dynamodb:GetItem", "dynamodb:PutItem",
                 "dynamodb:UpdateItem", "dynamodb:DeleteItem"],
      "Resource": "arn:aws:dynamodb:us-east-1:123456789012:table/relaymail-idempotency"
    }
  ]
}
```

A Terraform example that provisions all required AWS resources is in
[`deploy/terraform/aws-relaymail-email-ses-example/`](deploy/terraform/aws-relaymail-email-ses-example/).

---

## Configuration reference

All configuration is via environment variables prefixed `RELAYMAIL_`.

| Variable | Required | Default | Description |
|---|---|---|---|
| `RELAYMAIL_AWS_REGION` | Yes | — | AWS region |
| `RELAYMAIL_SQS_QUEUE_URL` | Yes | — | SQS queue URL to long-poll |
| `RELAYMAIL_S3_BUCKET_ALLOWLIST` | Yes | — | Comma-separated list of allowed S3 buckets |
| `RELAYMAIL_IDEMPOTENCY_TABLE_NAME` | No | — | DynamoDB table name; in-memory store used if unset (dev only) |
| `RELAYMAIL_DRY_RUN` | No | `false` | Validate but skip SES send |
| `RELAYMAIL_LOG_LEVEL` | No | `info` | Tracing level filter |
| `RELAYMAIL_LOG_JSON` | No | `false` | Emit JSON logs (enable in production) |
| `RELAYMAIL_WORKER_CONCURRENCY` | No | `4` | Parallel message processing slots |
| `RELAYMAIL_HTTP_BIND_ADDR` | No | `0.0.0.0:8080` | Address for health/metrics HTTP server |
| `RELAYMAIL_AWS_ENDPOINT_URL` | No | — | Override AWS endpoint (LocalStack) |

See [`examples/config/relaymail-email-ses.env.example`](examples/config/relaymail-email-ses.env.example)
for the full list.

---

## Build & test

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Coverage:

```sh
cargo llvm-cov --workspace --all-features --fail-under-lines 70
```

---

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
- Env-var prefix: `RELAYMAIL_`
- Metrics namespace: `relaymail_*`
- S3 tag keys: `relaymail-*`
