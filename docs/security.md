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
  the idempotency table

## Credentials

- **EKS**: use IRSA. Do not bake AWS access keys into the image or
  Kubernetes Secret. The provided ServiceAccount manifest ships with
  a `REPLACE_ME` placeholder for `eks.amazonaws.com/role-arn`.
- **EC2 / Fargate**: use instance profile or task role.
- **Local dev**: use the AWS SDK default credential chain (env vars,
  `~/.aws/credentials`, etc.). Point at LocalStack by setting
  `RELAYMAIL_AWS_ENDPOINT_URL=http://localhost:4566`.

## SES deliverability

RelayMail does not manage identity verification — do it outside the
service. Before sending real mail, confirm:

- [ ] SES identity (domain or address) is verified
- [ ] DKIM published as a CNAME set per SES console
- [ ] SPF record lists `include:amazonses.com`
- [ ] DMARC policy is published (`p=none` initially, then tighten)
- [ ] SES sending quota is out of the sandbox
- [ ] Configuration set wired for engagement tracking if needed

Bounce and complaint handling is a future `relaymail-events` concern —
until that ships, configure SES notifications to an SNS topic you
monitor separately.

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
- Container runs as non-root with `readOnlyRootFilesystem: true` and
  `capabilities.drop: [ALL]`.

## Supply chain

- Workspace forbids `unsafe_code`.
- CI denies clippy warnings.
- Every dependency is pinned in the workspace root `[workspace.dependencies]`.
