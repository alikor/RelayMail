# relaymail-api (future)

Planned HTTP/gRPC API for submitting outbound messages to RelayMail from
internal services.

**Not implemented in this phase.** See [FUTURE.md](FUTURE.md).

Integration plan:

- Accept submissions, validate via `relaymail-email`, persist to the
  object store under the conventional prefix, and rely on the existing
  S3 → SQS email worker to deliver.
- Reuse `relaymail-runtime` for health/metrics/shutdown/tracing.
- Authenticate via IAM + in-cluster service identity.
