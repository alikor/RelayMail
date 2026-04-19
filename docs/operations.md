# Operations runbook — relaymail-email-ses

## Key metrics

| Metric | Meaning |
|---|---|
| `relaymail_emails_processed_total{status}` | One increment per envelope outcome (`sent`, `skipped`, `dry_run`) |
| `relaymail_emails_sent_total` | Successful SES sends |
| `relaymail_email_failures_total{error_class}` | Failures bucketed by class |
| `relaymail_idempotency_skips_total` | Envelopes skipped because a prior run already sent |
| `relaymail_ses_send_latency_seconds` | Histogram — SES call time |
| `relaymail_processing_duration_seconds` | Histogram — end-to-end per-envelope time |

## Suggested SLOs (starting point)

- p99 end-to-end processing latency < 5s
- failure rate (`error_class != transient`) < 0.1% over rolling 30min
- SQS `ApproximateAgeOfOldestMessage` < 60s

## Common alerts

- Sustained `error_class=transient` > 5min → likely AWS throttling or a
  misconfigured IAM role. Check SES sending quota and worker logs.
- Messages in DLQ > 0 → review worker logs for the message ID; classify
  whether it's a bad fixture or a genuine code bug.
- Pod `Ready` flapping → likely IRSA / DNS / TCP issue; readiness gate
  waits on AWS client construction.

## Operator actions

| Intent | Command / steps |
|---|---|
| Drain one pod | `kubectl delete pod <name>` — liveness and PDB keep others up |
| Pause processing entirely | `kubectl scale deploy relaymail-email-ses --replicas=0` |
| Re-process an object | Re-upload it (new `eTag` → new idempotency key) |
| Inspect idempotency | Query the DynamoDB table with the object's `idempotency_key` |
| Move a failed object back to `incoming/` | `aws s3 mv s3://.../failed/<key> s3://.../incoming/<key>` |

## Disposition reference

Configured via `RELAYMAIL_PROCESSING_SUCCESS_MODE` and
`RELAYMAIL_PROCESSING_FAILURE_MODE`:

- `tag` — add `relaymail-status=...` plus timestamps and message IDs
- `move` — copy to `<prefix>/<key>` and delete the original
- `delete` — delete the original (success only)
- `none` — leave the object untouched

Tag mode is safest for auditing and re-running; move mode is tidier for
operational review.
