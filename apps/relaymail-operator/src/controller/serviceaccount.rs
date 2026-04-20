use std::collections::BTreeMap;

use k8s_openapi::api::core::v1::ServiceAccount;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::{Api, api::{Patch, PatchParams}};

use crate::crd::RelayMailSes;
use crate::error::Result;
use super::owner_ref::{owner_ref, resource_labels, sa_name};

pub async fn reconcile(obj: &RelayMailSes, client: &kube::Client, ns: &str) -> Result<()> {
    if !obj.spec.service_account.create {
        return Ok(());
    }
    let sa = build(obj)?;
    let api: Api<ServiceAccount> = Api::namespaced(client.clone(), ns);
    let params = PatchParams::apply("relaymail-operator").force();
    api.patch(&sa_name(obj), &params, &Patch::Apply(&sa))
        .await?;
    Ok(())
}

fn build(obj: &RelayMailSes) -> Result<ServiceAccount> {
    let annotations = obj
        .spec
        .service_account
        .irsa_role_arn
        .as_ref()
        .filter(|arn| !arn.is_empty())
        .map(|arn| {
            BTreeMap::from([(
                "eks.amazonaws.com/role-arn".to_string(),
                arn.clone(),
            )])
        });

    Ok(ServiceAccount {
        metadata: ObjectMeta {
            name: Some(sa_name(obj)),
            namespace: obj.metadata.namespace.clone(),
            labels: Some(resource_labels(obj)),
            annotations,
            owner_references: Some(vec![owner_ref(obj)?]),
            ..Default::default()
        },
        ..Default::default()
    })
}
