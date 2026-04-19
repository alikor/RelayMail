# relaymail-email-ses

RelayMail worker that long-polls Amazon SQS for S3 `ObjectCreated`
events, fetches each `.eml` / `.emi` object, validates it as raw
RFC-5322 mail, and sends it via Amazon SES v2 `SendEmail` (raw mode).

## Pipeline

```
S3 bucket
  -> S3 event (direct | SNS-wrapped | EventBridge)
  -> SQS queue
  -> relaymail-email-ses worker
  -> S3 object fetch
  -> raw MIME validation
  -> idempotency claim (DynamoDB or in-memory)
  -> Amazon SES v2 raw send
  -> S3 tag / move / delete / no-op disposition
  -> SQS message delete only after safe completion
```

## Configuration

All knobs live in environment variables with the `RELAYMAIL_` prefix.
See [../../examples/config/relaymail-email-ses.env.example](../../examples/config/relaymail-email-ses.env.example)
for the full list, and [src/config](src/config) for how they map to
typed sub-configs.

Required:
- `RELAYMAIL_AWS_REGION`
- `RELAYMAIL_SQS_QUEUE_URL`

Strongly recommended in production:
- `RELAYMAIL_IDEMPOTENCY_TABLE_NAME` (DynamoDB) — otherwise the worker
  uses an **in-memory dedupe cache that is not safe across restarts
  or multiple replicas** and logs a warning.

## AWS setup checklist

- [ ] S3 bucket exists
- [ ] Bucket notification → SQS for `s3:ObjectCreated:*` scoped to the
      `.eml` / `.emi` prefix
- [ ] SQS main queue + DLQ + redrive policy (`maxReceiveCount=5` is a
      good starting point)
- [ ] SES identity verified (domain or from-address)
- [ ] DKIM published, SPF aligned, DMARC policy
- [ ] DynamoDB table with `idempotency_key` hash key and TTL on `ttl`
- [ ] IRSA role with minimum IAM (see the Terraform example)

## Docker

```sh
docker build \
  -f deploy/docker/Dockerfile.relaymail-email-ses \
  -t relaymail-email-ses:dev .
```

## Kubernetes

```sh
kubectl apply -k deploy/k8s/relaymail-email-ses
```

## Failure handling

- **Transient** failures (AWS throttles, network, SES temporary
  rejection): the worker does **not** delete the SQS message. Visibility
  timeout expires and SQS redelivers; redrive policy sends to DLQ after
  `maxReceiveCount`.
- **Permanent** failures (invalid MIME, missing headers, oversized,
  SES permanent reject): the worker tags the object with
  `relaymail-status=failed` + `relaymail-error-class=<class>` and acks
  the SQS message.

## Idempotency limitations

See [../../docs/idempotency.md](../../docs/idempotency.md) for the
crash-window caveat and operational reconciliation guidance.

## Troubleshooting

| Symptom | Look at |
|---|---|
| Worker doesn't pick up new files | SQS queue metrics, S3 bucket notification config, `RELAYMAIL_S3_BUCKET_ALLOWLIST` |
| Messages stuck in SQS | Worker logs — look for `send failed` with a `transient` error class |
| Duplicate SES sends | Is `RELAYMAIL_IDEMPOTENCY_TABLE_NAME` set in prod? Review DDB table TTL |
| 503 on `/readyz` | Worker hasn't finished starting; check pod logs for startup errors |
| No metrics scraped | Prometheus target scraping `/metrics` on port 8080 |
