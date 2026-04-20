use k8s_openapi::api::core::v1::{Service, ServicePort, ServiceSpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use kube::{Api, api::{Patch, PatchParams}, ResourceExt};

use crate::crd::RelayMailSes;
use crate::error::Result;
use super::owner_ref::{owner_ref, resource_labels, selector_labels};

pub async fn reconcile(obj: &RelayMailSes, client: &kube::Client, ns: &str) -> Result<()> {
    let svc = build(obj)?;
    let api: Api<Service> = Api::namespaced(client.clone(), ns);
    let params = PatchParams::apply("relaymail-operator").force();
    api.patch(&obj.name_any(), &params, &Patch::Apply(&svc))
        .await?;
    Ok(())
}

fn build(obj: &RelayMailSes) -> Result<Service> {
    Ok(Service {
        metadata: ObjectMeta {
            name: Some(obj.name_any()),
            namespace: obj.metadata.namespace.clone(),
            labels: Some(resource_labels(obj)),
            owner_references: Some(vec![owner_ref(obj)?]),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            type_: Some("ClusterIP".to_string()),
            selector: Some(selector_labels(obj)),
            ports: Some(vec![ServicePort {
                name: Some("http".to_string()),
                port: 8080,
                target_port: Some(IntOrString::String("http".to_string())),
                protocol: Some("TCP".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    })
}
