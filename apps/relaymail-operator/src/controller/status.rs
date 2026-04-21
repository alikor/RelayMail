use std::sync::Arc;

use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::{
    api::{Patch, PatchParams},
    Api, ResourceExt,
};
use serde_json::json;

use crate::crd::RelayMailSes;
use crate::error::Result;

pub async fn patch_ready(
    obj: &Arc<RelayMailSes>,
    client: &kube::Client,
    ns: &str,
    ready: bool,
    message: Option<String>,
) -> Result<()> {
    let api: Api<RelayMailSes> = Api::namespaced(client.clone(), ns);
    let patch = json!({
        "apiVersion": "relaymail.io/v1alpha1",
        "kind": "RelayMailSes",
        "metadata": ObjectMeta {
            name: Some(obj.name_any()),
            namespace: Some(ns.to_string()),
            ..Default::default()
        },
        "status": {
            "ready": ready,
            "message": message,
        }
    });
    let params = PatchParams::apply("relaymail-operator").force();
    api.patch_status(&obj.name_any(), &params, &Patch::Apply(&patch))
        .await?;
    Ok(())
}
