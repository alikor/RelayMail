# Top-level marker file for the example module. The interesting resources
# live in sibling .tf files (iam.tf, sqs.tf, s3-notifications.tf, dynamodb.tf).
#
# To use:
#   terraform init
#   terraform apply \
#     -var 'inbound_bucket=my-inbound-bucket' \
#     -var 'name_prefix=relaymail'
#
# Attach the IAM policy emitted as `worker_policy_arn` to the IAM role that
# your Kubernetes ServiceAccount (IRSA) assumes.
