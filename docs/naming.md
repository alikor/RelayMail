# Naming conventions

## Capability

- Human name: **RelayMail**
- Repository: `RelayMail` (GitHub), module identifier `relaymail` (lowercase)

## Services

| Role | Human name | Binary / package | Status |
|---|---|---|---|
| Email worker | RelayMail Email Worker | `relaymail-email-ses` | Implemented |
| REST provider adapters | RelayMail Providers | `relaymail-providers` | Implemented |
| Direct SMTP/MTA | RelayMail Direct MTA | `relaymail-direct-mta` | Placeholder |
| Submission API | RelayMail API | `relaymail-api` | Placeholder |
| Delivery-event pipeline | RelayMail Events | `relaymail-events` | Placeholder |
| Operator console | RelayMail Console | `relaymail-console` | Placeholder |

## Crates

Library crates live under `crates/`. Each is named `relaymail-<role>` and
corresponds to exactly one concern:

- `relaymail-core` — domain types + capability traits (no adapter deps)
- `relaymail-email` — raw-MIME, validation, redaction
- `relaymail-delivery` — `EmailSender` trait + result/error
- `relaymail-aws` — AWS SDK adapter impls
- `relaymail-providers` — Resend, Postmark, and SMTP2GO REST adapters
- `relaymail-runtime` — HTTP health/metrics, tracing, worker, pipeline
- `relaymail-testing` — fakes + fixtures (not published)

## Env vars

- Prefix: `RELAYMAIL_`
- Name style: `RELAYMAIL_<CONCERN>_<SETTING>`, e.g.
  `RELAYMAIL_SQS_QUEUE_URL`, `RELAYMAIL_IDEMPOTENCY_TABLE_NAME`.

## Metrics

- Namespace: `relaymail_`
- Label keys: `service`, `provider`, `status`, `error_class`, `tenant`

## S3 tags

Tag keys RelayMail writes start with `relaymail-`:

- `relaymail-status` — `sent` | `failed`
- `relaymail-service` — e.g. `relaymail-email-ses`
- `relaymail-provider` — e.g. `resend`
- `relaymail-provider-message-id`
- `relaymail-error-class`
- `relaymail-processed-at`

## Log targets

Each binary uses a tracing target matching its binary name with dots for
crate hierarchy, e.g. `relaymail_email_ses::dry_run`.
