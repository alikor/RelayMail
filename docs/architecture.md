# Architecture

RelayMail is a Rust Cargo workspace of reusable capability crates plus
adapter crates plus one runnable worker. Domain crates never depend on
adapter (AWS SDK, HTTP, framework) types, so future services plug in
without disturbing the core.

## Crate graph

```
relaymail-core <- relaymail-email <- relaymail-delivery
       ^                ^                  ^
       |                |                  |
       +-------- relaymail-aws ------------+
       +-------- relaymail-runtime
       +-------- relaymail-testing
                                  |
                                  v
                apps/relaymail-email-ses  (composition root)
                         |
                         v
                relaymail-providers (REST adapters)
```

Capability traits live in the crate that consumes them. Implementations
live in adapter crates (AWS, testing) and are wired in the binary only.

## Current provider-chain flow

```mermaid
flowchart LR
    bucket[S3 bucket]
    providers[Resend / Postmark / SMTP2GO / optional SES]
    dlq[SQS DLQ]

    bucket -- ObjectCreated event --> sqs[SQS main queue]
    sqs -->|long poll| worker[relaymail-email-ses]
    worker -->|HEAD+GET| bucket
    worker -->|validate MIME| worker
    worker -->|claim| ddb[(DynamoDB idempotency)]
    worker -->|provider chain send| providers
    worker -->|tag: relaymail-status=sent| bucket
    worker -->|DeleteMessage| sqs
    sqs -. maxReceiveCount exceeded .-> dlq
```

## Failure & retry flow

```mermaid
flowchart LR
    in[Envelope] --> parse{Event parse}
    parse -- unknown --> dropAck[Ack without work]
    parse -- ok --> filter{Bucket / prefix / ext filter}
    filter -- skip --> dropAck
    filter -- pass --> claim{Idempotency claim}
    claim -- already-sent --> skipAck[Ack without work]
    claim -- transient --> nack[Nack: let visibility expire]
    claim -- proceed --> send{Send}
    send -- transient --> nack
    send -- permanent --> tagFail[Tag failed + ack]
    send -- accepted --> tagSent[Tag sent + ack]
```

## Provider architecture

```mermaid
flowchart LR
    subgraph relaymail-runtime
      pipeline[Pipeline stages]
    end
    subgraph relaymail-delivery
      iface[[EmailSender]]
    end
    pipeline --> iface
    iface --> chain[RelayMailDeliveryService]
    chain --> resend[relaymail-providers::ResendSender]
    chain --> postmark[relaymail-providers::PostmarkSender]
    chain --> smtp2go[relaymail-providers::Smtp2GoSender]
    chain --> ses[relaymail-aws::ses SesSender]
```

## Future direct MTA architecture

See [future-direct-mta.md](future-direct-mta.md). Short version: the new
`relaymail-direct-mta` service implements `EmailSender` against a
self-managed SMTP stack, and reuses `relaymail-runtime`'s pipeline.

## Observability

- **Logs**: JSON-structured, one event per processed message with
  `service`, `environment`, `tenant_id`, `provider`, `sqs_message_id`,
  `bucket`, key hash, size, idempotency-key hash, provider message id,
  error class. Never the body, never full recipient addresses.
- **Metrics**: Prometheus over `/metrics`. See
  [docs/operations.md](operations.md).
- **Health**: `GET /healthz` (process alive), `GET /readyz` (config +
  AWS clients + pipeline initialized).
