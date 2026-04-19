data "aws_iam_policy_document" "worker" {
  statement {
    sid    = "S3Read"
    effect = "Allow"
    actions = [
      "s3:GetObject",
      "s3:GetObjectTagging",
      "s3:PutObjectTagging",
      "s3:CopyObject",
      "s3:DeleteObject",
    ]
    resources = ["arn:aws:s3:::${var.inbound_bucket}/*"]
  }

  statement {
    sid    = "Sqs"
    effect = "Allow"
    actions = [
      "sqs:ReceiveMessage",
      "sqs:DeleteMessage",
      "sqs:ChangeMessageVisibility",
      "sqs:GetQueueAttributes",
    ]
    resources = [aws_sqs_queue.relaymail_inbound.arn]
  }

  statement {
    sid       = "Ses"
    effect    = "Allow"
    actions   = ["ses:SendEmail", "ses:SendRawEmail"]
    resources = ["*"]
  }

  dynamic "statement" {
    for_each = var.enable_idempotency_table ? [1] : []
    content {
      sid    = "Dynamo"
      effect = "Allow"
      actions = [
        "dynamodb:GetItem",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
      ]
      resources = [aws_dynamodb_table.relaymail_idempotency[0].arn]
    }
  }
}

resource "aws_iam_policy" "worker" {
  name   = "${var.name_prefix}-email-ses-worker"
  policy = data.aws_iam_policy_document.worker.json
}
