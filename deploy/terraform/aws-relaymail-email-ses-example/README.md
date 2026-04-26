# RelayMail Email Worker — AWS example

Minimal Terraform that provisions everything the worker needs, *except*
the S3 bucket itself and provider sender-domain verification records
(both have real blast radius — manage them separately).

## What this creates

- SQS main queue + DLQ with a redrive policy
- S3 → SQS bucket notification scoped to `incoming/*.eml` and `*.emi`
- DynamoDB table for idempotency (optional)
- DynamoDB table for transport state, provider events, and suppressions
  (optional)
- IAM policy with the minimum actions the worker needs

## What you must provide

- A pre-existing S3 bucket (the one RelayMail watches)
- Verified sender domains with each enabled provider
- Provider API keys and webhook secrets through your secret manager
- An IRSA-enabled EKS cluster (or an EC2 instance profile) to assume
  the IAM policy emitted here

## Apply

```sh
terraform init
terraform apply \
  -var 'inbound_bucket=my-inbound-bucket' \
  -var 'name_prefix=relaymail'
```

Feed the outputs into your Helm values or Kubernetes ConfigMap:

```
RELAYMAIL_SQS_QUEUE_URL      = terraform output -raw sqs_queue_url
RELAYMAIL_IDEMPOTENCY_TABLE_NAME = terraform output -raw idempotency_table_name
RELAYMAIL_TRANSPORT_STATE_TABLE_NAME = terraform output -raw transport_state_table_name
serviceAccount.annotations.'eks.amazonaws.com/role-arn' = <your IRSA role ARN>
```

No secrets are committed.
