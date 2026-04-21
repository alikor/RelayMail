pub mod configmap;
pub mod deployment;
pub mod hpa;
pub mod owner_ref;
pub mod pdb;
pub mod reconciler;
pub mod service;
pub mod serviceaccount;
pub mod status;

use std::sync::Arc;

use anyhow::Result;
use futures::StreamExt;
use kube::{
    runtime::{controller::Controller, watcher},
    Api,
};

use crate::crd::RelayMailSes;

pub struct Context {
    pub client: kube::Client,
}

impl Context {
    pub fn new(client: kube::Client) -> Self {
        Self { client }
    }
}

pub async fn run(ctx: Arc<Context>) -> Result<()> {
    let api: Api<RelayMailSes> = Api::all(ctx.client.clone());
    Controller::new(api, watcher::Config::default())
        .run(reconciler::reconcile, reconciler::error_policy, ctx)
        .for_each(|result| async move {
            match result {
                Ok(obj) => tracing::debug!("reconciled {:?}", obj),
                Err(e) => tracing::warn!("reconcile error: {:?}", e),
            }
        })
        .await;
    Ok(())
}
