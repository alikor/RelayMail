resource "aws_sqs_queue" "relaymail_dlq" {
  name                      = "${var.name_prefix}-inbound-dlq"
  message_retention_seconds = 1209600
}

resource "aws_sqs_queue" "relaymail_inbound" {
  name                       = "${var.name_prefix}-inbound"
  visibility_timeout_seconds = 300
  message_retention_seconds  = 345600
  receive_wait_time_seconds  = 20

  redrive_policy = jsonencode({
    deadLetterTargetArn = aws_sqs_queue.relaymail_dlq.arn
    maxReceiveCount     = 5
  })
}

data "aws_iam_policy_document" "inbound_queue_policy" {
  statement {
    sid     = "AllowS3ToPublish"
    effect  = "Allow"
    actions = ["sqs:SendMessage"]
    principals {
      type        = "Service"
      identifiers = ["s3.amazonaws.com"]
    }
    resources = [aws_sqs_queue.relaymail_inbound.arn]
    condition {
      test     = "ArnLike"
      variable = "aws:SourceArn"
      values   = ["arn:aws:s3:::${var.inbound_bucket}"]
    }
  }
}

resource "aws_sqs_queue_policy" "inbound_queue_policy" {
  queue_url = aws_sqs_queue.relaymail_inbound.id
  policy    = data.aws_iam_policy_document.inbound_queue_policy.json
}
