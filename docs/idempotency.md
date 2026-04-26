# Idempotency

## Key formula

```
idempotency_key = sha256( tenant_id | bucket | key | version_or_etag | size_bytes )
                  (hex-encoded, 64 chars)
```

Computed in [`relaymail-core/src/idempotency/key.rs`](../crates/relaymail-core/src/idempotency/key.rs).
Two attempts over the **same S3 object version** hash to the same key;
a re-upload produces a different `eTag` (and usually `size_bytes`), so
it is treated as a new message — this is intentional.

`tenant_id` is empty when `RELAYMAIL_TENANT_ID` is unset, which is fine
for single-tenant deployments.

## DynamoDB schema

| Attribute | Type | Notes |
|---|---|---|
| `idempotency_key` (PK) | S | the hex digest above |
| `status` | S | `processing`, `sent`, or `failed` |
| `created_at` | S | RFC 3339 — when the claim was first written |
| `ttl` | N | epoch seconds, consumed by DynamoDB TTL |
| `detail` | S | provider message ID (on sent) or reason (on failed) |

TTL is configured via `RELAYMAIL_IDEMPOTENCY_TTL_SECONDS` (default
604800 — 7 days).

## Claim protocol

1. Worker computes `idempotency_key`.
2. Claim: conditional `PutItem` with
   `attribute_not_exists(idempotency_key)`.
   - Success → proceed with send.
   - `ConditionalCheckFailedException` → another worker already claimed
     this key. Read the existing record; skip if `status=sent`; treat
     `processing`/`failed` as "already claimed, do not send again".
3. After SES returns success, worker calls `UpdateItem` to set
   `status=sent` and the provider message ID.
4. On permanent failure, worker calls `UpdateItem` to set
   `status=failed` and the reason.

## Crash-window caveat

SES's raw `SendEmail` does **not** expose a universal idempotency key
for arbitrary raw messages. If the worker crashes after SES accepted
the message but before `UpdateItem` (step 3) completes, a retry will
re-claim (because the record is still `processing`) and send again,
producing a duplicate delivery.

Mitigations RelayMail applies today:

- `UpdateItem` to `sent` runs immediately after SES success — the
  window is typically milliseconds.
- The object tag `relaymail-provider-message-id=<id>` is written right after
  the `UpdateItem`, so operators can reconcile by SES message id if
  they see unexpected duplicates.
- Move mode (`RELAYMAIL_PROCESSING_SUCCESS_MODE=move`) makes reprocess
  safer because the object leaves the watched prefix.

Stronger guarantees (outbox pattern, two-phase sends) are future work —
tracked for when `relaymail-direct-mta` lands because a self-managed MTA
provides better reissue controls.

## In-memory fallback

If `RELAYMAIL_IDEMPOTENCY_TABLE_NAME` is unset, the worker falls back
to `InMemoryIdempotencyStore`. This is **not safe across restarts or
multiple replicas** and the worker logs a loud `WARN` at startup. Use
only for local dev, unit tests, and LocalStack experiments.
