# Future scope — relaymail-api

- Submission API (HTTP JSON or gRPC) with strict validation.
- Multi-tenant auth, rate limiting, quota tracking.
- Emits the same raw `.eml` format to the object store so the existing
  worker service doesn't need to know about the API path.
- Keep parser shared with `relaymail-email` so validation stays
  consistent between ingestion paths.

No code in this directory yet.
