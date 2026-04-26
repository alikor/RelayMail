variable "aws_region" {
  type        = string
  description = "AWS region."
  default     = "us-east-1"
}

variable "name_prefix" {
  type        = string
  description = "Prefix for RelayMail resources."
  default     = "relaymail"
}

variable "inbound_bucket" {
  type        = string
  description = "Existing S3 bucket RelayMail watches. Create it outside this module."
}

variable "eml_prefix" {
  type        = string
  description = "Prefix under which new objects trigger processing."
  default     = "incoming/"
}

variable "enable_idempotency_table" {
  type        = bool
  description = "Create a DynamoDB table for at-most-once delivery."
  default     = true
}

variable "enable_transport_state_table" {
  type        = bool
  description = "Create a DynamoDB table for send attempts, provider events, and suppressions."
  default     = true
}
