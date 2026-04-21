use k8s_openapi::api::autoscaling::v2::{
    HorizontalPodAutoscaler, HorizontalPodAutoscalerSpec, MetricSpec, MetricTarget,
    ResourceMetricSource,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::{
    api::{DeleteParams, Patch, PatchParams},
    Api, ResourceExt,
};

use super::owner_ref::{owner_ref, resource_labels};
use crate::crd::RelayMailSes;
use crate::error::Result;

pub async fn reconcile(obj: &RelayMailSes, client: &kube::Client, ns: &str) -> Result<()> {
    let api: Api<HorizontalPodAutoscaler> = Api::namespaced(client.clone(), ns);
    if !obj.spec.hpa.enabled {
        // Remove HPA if it exists and HPA is disabled.
        if api.get_opt(&obj.name_any()).await?.is_some() {
            api.delete(&obj.name_any(), &DeleteParams::default())
                .await?;
        }
        return Ok(());
    }
    let hpa = build(obj)?;
    let params = PatchParams::apply("relaymail-operator").force();
    api.patch(&obj.name_any(), &params, &Patch::Apply(&hpa))
        .await?;
    Ok(())
}

fn build(obj: &RelayMailSes) -> Result<HorizontalPodAutoscaler> {
    let hpa = &obj.spec.hpa;
    Ok(HorizontalPodAutoscaler {
        metadata: ObjectMeta {
            name: Some(obj.name_any()),
            namespace: obj.metadata.namespace.clone(),
            labels: Some(resource_labels(obj)),
            owner_references: Some(vec![owner_ref(obj)?]),
            ..Default::default()
        },
        spec: Some(HorizontalPodAutoscalerSpec {
            scale_target_ref: k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference {
                api_version: Some("apps/v1".to_string()),
                kind: "Deployment".to_string(),
                name: obj.name_any(),
            },
            min_replicas: Some(hpa.min_replicas),
            max_replicas: hpa.max_replicas,
            metrics: Some(vec![MetricSpec {
                type_: "Resource".to_string(),
                resource: Some(ResourceMetricSource {
                    name: "cpu".to_string(),
                    target: MetricTarget {
                        type_: "Utilization".to_string(),
                        average_utilization: Some(hpa.target_cpu_utilization_percentage),
                        ..Default::default()
                    },
                }),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    })
}
