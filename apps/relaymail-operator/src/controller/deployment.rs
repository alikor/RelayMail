use std::collections::BTreeMap;

use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec};
use k8s_openapi::api::core::v1::{
    Capabilities, ConfigMapEnvSource, Container, ContainerPort, EnvFromSource,
    HTTPGetAction, PodSecurityContext, PodSpec, PodTemplateSpec, Probe,
    ResourceRequirements, SecurityContext,
};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use kube::{Api, ResourceExt, api::{Patch, PatchParams}};

use crate::crd::RelayMailSes;
use crate::error::Result;
use super::owner_ref::{configmap_name, owner_ref, resource_labels, sa_name, selector_labels};

pub async fn reconcile(obj: &RelayMailSes, client: &kube::Client, ns: &str) -> Result<()> {
    let dep = build(obj)?;
    let api: Api<Deployment> = Api::namespaced(client.clone(), ns);
    let params = PatchParams::apply("relaymail-operator").force();
    api.patch(&obj.name_any(), &params, &Patch::Apply(&dep))
        .await?;
    Ok(())
}

fn build(obj: &RelayMailSes) -> Result<Deployment> {
    let sel = selector_labels(obj);
    let labels = resource_labels(obj);
    let image = format!("{}:{}", obj.spec.image.repository, obj.spec.image.tag);
    let res = &obj.spec.resources;

    let resources = ResourceRequirements {
        requests: Some(resource_map(&res.requests.cpu, &res.requests.memory)),
        limits: Some(resource_map(&res.limits.cpu, &res.limits.memory)),
        ..Default::default()
    };

    Ok(Deployment {
        metadata: ObjectMeta {
            name: Some(obj.name_any()),
            namespace: obj.metadata.namespace.clone(),
            labels: Some(labels.clone()),
            owner_references: Some(vec![owner_ref(obj)?]),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(obj.spec.replicas),
            selector: LabelSelector {
                match_labels: Some(sel.clone()),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(labels),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    service_account_name: Some(sa_name(obj)),
                    security_context: Some(PodSecurityContext {
                        run_as_non_root: Some(true),
                        ..Default::default()
                    }),
                    containers: vec![Container {
                        name: "worker".to_string(),
                        image: Some(image),
                        image_pull_policy: Some(obj.spec.image.pull_policy.clone()),
                        ports: Some(vec![ContainerPort {
                            name: Some("http".to_string()),
                            container_port: 8080,
                            protocol: Some("TCP".to_string()),
                            ..Default::default()
                        }]),
                        env_from: Some(vec![EnvFromSource {
                            config_map_ref: Some(ConfigMapEnvSource {
                                name: Some(configmap_name(obj)),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }]),
                        security_context: Some(SecurityContext {
                            allow_privilege_escalation: Some(false),
                            read_only_root_filesystem: Some(true),
                            capabilities: Some(Capabilities {
                                drop: Some(vec!["ALL".to_string()]),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        liveness_probe: Some(http_probe("/healthz", 5, 10)),
                        readiness_probe: Some(http_probe("/readyz", 5, 10)),
                        resources: Some(resources),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    })
}

fn resource_map(cpu: &str, memory: &str) -> BTreeMap<String, Quantity> {
    BTreeMap::from([
        ("cpu".to_string(), Quantity(cpu.to_string())),
        ("memory".to_string(), Quantity(memory.to_string())),
    ])
}

fn http_probe(path: &str, initial_delay: i32, period: i32) -> Probe {
    Probe {
        http_get: Some(HTTPGetAction {
            path: Some(path.to_string()),
            port: IntOrString::String("http".to_string()),
            ..Default::default()
        }),
        initial_delay_seconds: Some(initial_delay),
        period_seconds: Some(period),
        timeout_seconds: Some(2),
        failure_threshold: Some(3),
        ..Default::default()
    }
}
