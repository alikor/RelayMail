resource "aws_s3_bucket_notification" "relaymail" {
  bucket = var.inbound_bucket

  queue {
    queue_arn     = aws_sqs_queue.relaymail_inbound.arn
    events        = ["s3:ObjectCreated:*"]
    filter_prefix = var.eml_prefix
    filter_suffix = ".eml"
  }

  queue {
    queue_arn     = aws_sqs_queue.relaymail_inbound.arn
    events        = ["s3:ObjectCreated:*"]
    filter_prefix = var.eml_prefix
    filter_suffix = ".emi"
  }

  depends_on = [aws_sqs_queue_policy.inbound_queue_policy]
}
