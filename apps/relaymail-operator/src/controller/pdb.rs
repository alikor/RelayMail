use k8s_openapi::api::policy::v1::{PodDisruptionBudget, PodDisruptionBudgetSpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use kube::{
    api::{Patch, PatchParams},
    Api, ResourceExt,
};

use super::owner_ref::{owner_ref, resource_labels, selector_labels};
use crate::crd::RelayMailSes;
use crate::error::Result;

pub async fn reconcile(obj: &RelayMailSes, client: &kube::Client, ns: &str) -> Result<()> {
    let pdb = build(obj)?;
    let api: Api<PodDisruptionBudget> = Api::namespaced(client.clone(), ns);
    let params = PatchParams::apply("relaymail-operator").force();
    api.patch(&obj.name_any(), &params, &Patch::Apply(&pdb))
        .await?;
    Ok(())
}

fn build(obj: &RelayMailSes) -> Result<PodDisruptionBudget> {
    Ok(PodDisruptionBudget {
        metadata: ObjectMeta {
            name: Some(obj.name_any()),
            namespace: obj.metadata.namespace.clone(),
            labels: Some(resource_labels(obj)),
            owner_references: Some(vec![owner_ref(obj)?]),
            ..Default::default()
        },
        spec: Some(PodDisruptionBudgetSpec {
            min_available: Some(IntOrString::Int(obj.spec.pdb.min_available)),
            selector: Some(LabelSelector {
                match_labels: Some(selector_labels(obj)),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    })
}
