output "sqs_queue_url" {
  value       = aws_sqs_queue.relaymail_inbound.id
  description = "Value to set as RELAYMAIL_SQS_QUEUE_URL."
}

output "sqs_queue_arn" {
  value = aws_sqs_queue.relaymail_inbound.arn
}

output "sqs_dlq_arn" {
  value = aws_sqs_queue.relaymail_dlq.arn
}

output "worker_policy_arn" {
  value       = aws_iam_policy.worker.arn
  description = "Attach this to the IRSA role for the worker pod."
}

output "idempotency_table_name" {
  value = var.enable_idempotency_table ? aws_dynamodb_table.relaymail_idempotency[0].name : null
}
