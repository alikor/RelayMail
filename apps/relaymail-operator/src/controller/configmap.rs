use std::collections::BTreeMap;

use k8s_openapi::api::core::v1::ConfigMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::{Api, api::{Patch, PatchParams}};

use crate::crd::RelayMailSes;
use crate::error::Result;
use super::owner_ref::{configmap_name, owner_ref, resource_labels};

pub async fn reconcile(obj: &RelayMailSes, client: &kube::Client, ns: &str) -> Result<()> {
    let cm = build(obj)?;
    let api: Api<ConfigMap> = Api::namespaced(client.clone(), ns);
    let params = PatchParams::apply("relaymail-operator").force();
    api.patch(&configmap_name(obj), &params, &Patch::Apply(&cm))
        .await?;
    Ok(())
}

fn build(obj: &RelayMailSes) -> Result<ConfigMap> {
    let cfg = &obj.spec.config;
    let mut data: BTreeMap<String, String> = BTreeMap::new();

    data.insert("RELAYMAIL_AWS_REGION".into(), cfg.aws_region.clone());
    data.insert("RELAYMAIL_SQS_QUEUE_URL".into(), cfg.sqs_queue_url.clone());
    data.insert(
        "RELAYMAIL_S3_BUCKET_ALLOWLIST".into(),
        cfg.s3_bucket_allowlist.clone(),
    );
    if let Some(table) = &cfg.idempotency_table_name {
        data.insert("RELAYMAIL_IDEMPOTENCY_TABLE_NAME".into(), table.clone());
    }
    data.insert(
        "RELAYMAIL_WORKER_CONCURRENCY".into(),
        cfg.worker_concurrency.clone(),
    );
    data.insert("RELAYMAIL_DRY_RUN".into(), cfg.dry_run.clone());
    data.insert("RELAYMAIL_LOG_LEVEL".into(), cfg.log_level.clone());
    data.insert("RELAYMAIL_LOG_JSON".into(), cfg.log_json.clone());
    if let Some(endpoint) = &cfg.aws_endpoint_url {
        data.insert("RELAYMAIL_AWS_ENDPOINT_URL".into(), endpoint.clone());
    }

    Ok(ConfigMap {
        metadata: ObjectMeta {
            name: Some(configmap_name(obj)),
            namespace: obj.metadata.namespace.clone(),
            labels: Some(resource_labels(obj)),
            owner_references: Some(vec![owner_ref(obj)?]),
            ..Default::default()
        },
        data: Some(data),
        ..Default::default()
    })
}
