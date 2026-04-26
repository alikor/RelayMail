use relaymail_core::object_store::{ObjectStore, TagSet};
use relaymail_core::ObjectId;

use super::config::{FailureDispositionMode, ProcessingConfig, SuccessDispositionMode};
use super::error::StageError;

pub(crate) async fn on_success(
    store: &dyn ObjectStore,
    cfg: &ProcessingConfig,
    object: &ObjectId,
    provider_label: &str,
    send_message_id: &str,
    processed_at: &str,
) -> Result<(), StageError> {
    match cfg.success_mode {
        SuccessDispositionMode::Tag => {
            let tags = success_tags(cfg, provider_label, send_message_id, processed_at);
            store.tag(object, &tags).await?;
        }
        SuccessDispositionMode::Move => {
            let dest = format!("{}{}", cfg.success_prefix, object.key());
            store.move_to(object, &dest).await?;
        }
        SuccessDispositionMode::Delete => {
            store.delete(object).await?;
        }
        SuccessDispositionMode::None => {}
    }
    Ok(())
}

pub(crate) async fn on_failure(
    store: &dyn ObjectStore,
    cfg: &ProcessingConfig,
    object: &ObjectId,
    error_class: &str,
    processed_at: &str,
) -> Result<(), StageError> {
    match cfg.failure_mode {
        FailureDispositionMode::Tag => {
            let tags = failure_tags(cfg, error_class, processed_at);
            store.tag(object, &tags).await?;
        }
        FailureDispositionMode::Move => {
            let dest = format!("{}{}", cfg.failure_prefix, object.key());
            store.move_to(object, &dest).await?;
        }
        FailureDispositionMode::None => {}
    }
    Ok(())
}

fn success_tags(cfg: &ProcessingConfig, provider_label: &str, msg_id: &str, ts: &str) -> TagSet {
    let mut tags = TagSet::new();
    tags.insert("relaymail-status", "sent");
    tags.insert("relaymail-service", cfg.service_name.clone());
    tags.insert("relaymail-provider", provider_label.to_string());
    tags.insert("relaymail-provider-message-id", truncate(msg_id, 256));
    tags.insert("relaymail-processed-at", ts.to_string());
    tags
}

fn failure_tags(cfg: &ProcessingConfig, error_class: &str, ts: &str) -> TagSet {
    let mut tags = TagSet::new();
    tags.insert("relaymail-status", "failed");
    tags.insert("relaymail-service", cfg.service_name.clone());
    tags.insert("relaymail-provider", cfg.provider_label.clone());
    tags.insert("relaymail-error-class", error_class.to_string());
    tags.insert("relaymail-processed-at", ts.to_string());
    tags
}

fn truncate(value: &str, max: usize) -> String {
    if value.len() <= max {
        value.to_string()
    } else {
        value[..max].to_string()
    }
}
