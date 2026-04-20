use std::collections::BTreeMap;

use k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference;
use kube::ResourceExt;

use crate::crd::RelayMailSes;
use crate::error::{Error, Result};

pub fn owner_ref(obj: &RelayMailSes) -> Result<OwnerReference> {
    Ok(OwnerReference {
        api_version: "relaymail.io/v1alpha1".to_string(),
        kind: "RelayMailSes".to_string(),
        name: obj.name_any(),
        uid: obj.uid().ok_or(Error::MissingUid("RelayMailSes"))?,
        controller: Some(true),
        block_owner_deletion: Some(true),
    })
}

pub fn selector_labels(obj: &RelayMailSes) -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "app.kubernetes.io/name".to_string(),
            "relaymail-email-ses".to_string(),
        ),
        (
            "app.kubernetes.io/instance".to_string(),
            obj.name_any(),
        ),
    ])
}

pub fn resource_labels(obj: &RelayMailSes) -> BTreeMap<String, String> {
    let mut labels = selector_labels(obj);
    labels.insert(
        "app.kubernetes.io/managed-by".to_string(),
        "relaymail-operator".to_string(),
    );
    labels.insert(
        "app.kubernetes.io/part-of".to_string(),
        "relaymail".to_string(),
    );
    labels
}

pub fn sa_name(obj: &RelayMailSes) -> String {
    format!("{}-worker", obj.name_any())
}

pub fn configmap_name(obj: &RelayMailSes) -> String {
    format!("{}-config", obj.name_any())
}
