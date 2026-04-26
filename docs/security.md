# Security

## Least-privilege IAM

The worker needs the actions listed in
[`deploy/terraform/aws-relaymail-email-ses-example/iam.tf`](../deploy/terraform/aws-relaymail-email-ses-example/iam.tf):

- `s3:GetObject`, `s3:GetObjectTagging`, `s3:PutObjectTagging`,
  `s3:CopyObject`, `s3:DeleteObject` — scoped to the inbound bucket
- `sqs:ReceiveMessage`, `sqs:DeleteMessage`, `sqs:ChangeMessageVisibility`,
  `sqs:GetQueueAttributes` — scoped to the worker's queue
- `ses:SendEmail`, `ses:SendRawEmail`
- `dynamodb:GetItem`, `PutItem`, `UpdateItem`, `DeleteItem` — scoped to
  the idempotency and transport-state tables

## Credentials

- **EKS**: use IRSA. Do not bake AWS access keys into the image or
  Kubernetes Secret. The provided ServiceAccount manifest ships with
  a `REPLACE_ME` placeholder for `eks.amazonaws.com/role-arn`.
- **EC2 / Fargate**: use instance profile or task role.
- **Local dev**: use the AWS SDK default credential chain (env vars,
  `~/.aws/credentials`, etc.). Point at LocalStack by setting
  `RELAYMAIL_AWS_ENDPOINT_URL=http://localhost:4566`.

## Provider Credentials

- Store Resend, Postmark, SMTP2GO, and optional SES credentials in the
  environment or a secret manager. Never commit populated values.
- Prefer least-privilege/scoped API keys where a provider supports them.
- Provider tokens are never logged, returned to clients, or written to
  object tags.

## Deliverability

RelayMail does not manage identity verification or DNS — do it outside
the service for every configured stream domain and provider. Before
sending real mail, confirm:

- [ ] Sender domain is verified in each enabled provider
- [ ] DKIM records are published for each provider/domain pair
- [ ] SPF includes the active providers for the stream domain
- [ ] DMARC policy is published (`p=none` initially, then tighten)
- [ ] Provider sending quotas/account approvals are complete
- [ ] Transactional and marketing streams use separate domains where
      desired
- [ ] Marketing messages include unsubscribe and consent metadata

Bounce, complaint, suppression, and unsubscribe webhooks are accepted at
`/api/relaymail/webhooks/{provider}` and feed RelayMail suppressions.

## Redaction and logs

- Email bodies, attachments, and full recipient addresses are **never**
  logged.
- Recipient logging uses `first-char + @ + domain` via
  [`relaymail-email/src/redaction/headers.rs`](../crates/relaymail-email/src/redaction/headers.rs).
- Idempotency keys appear in logs as a 16-char prefix only.

## Secrets

- No secrets in the repo.
- `deploy/k8s/.../secret.template.yaml` is a template only and has an
  explicit "never commit a populated version" comment.
- Webhook endpoints require provider-specific authentication before
  payload parsing.
- Sender domains and provider selection are configuration-controlled, not
  arbitrary user input.
- Container runs as non-root with `readOnlyRootFilesystem: true` and
  `capabilities.drop: [ALL]`.

## Supply chain

- Workspace forbids `unsafe_code`.
- CI denies clippy warnings.
- Every dependency is pinned in the workspace root `[workspace.dependencies]`.
