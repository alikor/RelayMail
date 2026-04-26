use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Manages a RelayMail email worker deployment and all its Kubernetes resources.
#[derive(CustomResource, Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
#[kube(
    group = "relaymail.io",
    version = "v1alpha1",
    kind = "RelayMailSes",
    plural = "relaymailseses",
    singular = "relaymailses",
    shortname = "rms",
    namespaced,
    status = "RelayMailSesStatus",
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.ready"}"#,
    printcolumn = r#"{"name":"Replicas","type":"integer","jsonPath":".status.readyReplicas"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct RelayMailSesSpec {
    #[serde(default)]
    pub image: ImageSpec,
    #[serde(default = "default_replicas")]
    pub replicas: i32,
    pub config: SesConfig,
    #[serde(default)]
    pub service_account: ServiceAccountSpec,
    #[serde(default)]
    pub resources: ResourceSpec,
    #[serde(default)]
    pub hpa: HpaSpec,
    #[serde(default)]
    pub pdb: PdbSpec,
}

fn default_replicas() -> i32 {
    2
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageSpec {
    #[serde(default = "default_repository")]
    pub repository: String,
    #[serde(default = "default_tag")]
    pub tag: String,
    #[serde(default = "default_pull_policy")]
    pub pull_policy: String,
}

fn default_repository() -> String {
    "alikor/relaymail".into()
}
fn default_tag() -> String {
    "latest".into()
}
fn default_pull_policy() -> String {
    "IfNotPresent".into()
}

impl Default for ImageSpec {
    fn default() -> Self {
        Self {
            repository: default_repository(),
            tag: default_tag(),
            pull_policy: default_pull_policy(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SesConfig {
    #[serde(default = "default_region")]
    pub aws_region: String,
    pub sqs_queue_url: String,
    pub s3_bucket_allowlist: String,
    #[serde(default)]
    pub idempotency_table_name: Option<String>,
    #[serde(default = "default_concurrency")]
    pub worker_concurrency: String,
    #[serde(default = "default_false")]
    pub dry_run: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_true")]
    pub log_json: String,
    #[serde(default)]
    pub aws_endpoint_url: Option<String>,
}

fn default_region() -> String {
    "us-east-1".into()
}
fn default_concurrency() -> String {
    "4".into()
}
fn default_false() -> String {
    "false".into()
}
fn default_true() -> String {
    "true".into()
}
fn default_log_level() -> String {
    "info".into()
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountSpec {
    #[serde(default = "default_sa_create")]
    pub create: bool,
    #[serde(default)]
    pub irsa_role_arn: Option<String>,
}

fn default_sa_create() -> bool {
    true
}

impl Default for ServiceAccountSpec {
    fn default() -> Self {
        Self {
            create: true,
            irsa_role_arn: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq, Default)]
pub struct ResourceSpec {
    #[serde(default)]
    pub requests: ResourceList,
    #[serde(default)]
    pub limits: ResourceList,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
pub struct ResourceList {
    #[serde(default = "default_cpu_req")]
    pub cpu: String,
    #[serde(default = "default_mem_req")]
    pub memory: String,
}

fn default_cpu_req() -> String {
    "100m".into()
}
fn default_mem_req() -> String {
    "128Mi".into()
}

impl Default for ResourceList {
    fn default() -> Self {
        Self {
            cpu: default_cpu_req(),
            memory: default_mem_req(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HpaSpec {
    #[serde(default = "default_sa_create")]
    pub enabled: bool,
    #[serde(default = "default_min_replicas")]
    pub min_replicas: i32,
    #[serde(default = "default_max_replicas")]
    pub max_replicas: i32,
    #[serde(default = "default_cpu_target")]
    pub target_cpu_utilization_percentage: i32,
}

fn default_min_replicas() -> i32 {
    2
}
fn default_max_replicas() -> i32 {
    10
}
fn default_cpu_target() -> i32 {
    70
}

impl Default for HpaSpec {
    fn default() -> Self {
        Self {
            enabled: true,
            min_replicas: default_min_replicas(),
            max_replicas: default_max_replicas(),
            target_cpu_utilization_percentage: default_cpu_target(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PdbSpec {
    #[serde(default = "default_min_available")]
    pub min_available: i32,
}

fn default_min_available() -> i32 {
    1
}

impl Default for PdbSpec {
    fn default() -> Self {
        Self {
            min_available: default_min_available(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelayMailSesStatus {
    pub ready: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
