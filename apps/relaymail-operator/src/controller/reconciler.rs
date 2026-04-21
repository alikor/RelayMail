use std::{sync::Arc, time::Duration};

use kube::{runtime::controller::Action, ResourceExt};

use super::{configmap, deployment, hpa, pdb, service, serviceaccount, status, Context};
use crate::crd::RelayMailSes;
use crate::error::{Error, Result};

pub async fn reconcile(obj: Arc<RelayMailSes>, ctx: Arc<Context>) -> Result<Action> {
    let ns = obj
        .namespace()
        .ok_or(Error::MissingNamespace("RelayMailSes"))?;
    let name = obj.name_any();

    tracing::info!(ns = %ns, name = %name, "reconciling RelayMailSes");

    serviceaccount::reconcile(&obj, &ctx.client, &ns).await?;
    configmap::reconcile(&obj, &ctx.client, &ns).await?;
    deployment::reconcile(&obj, &ctx.client, &ns).await?;
    service::reconcile(&obj, &ctx.client, &ns).await?;
    hpa::reconcile(&obj, &ctx.client, &ns).await?;
    pdb::reconcile(&obj, &ctx.client, &ns).await?;

    status::patch_ready(&obj, &ctx.client, &ns, true, None).await?;

    tracing::info!(ns = %ns, name = %name, "reconcile complete");
    Ok(Action::requeue(Duration::from_secs(300)))
}

pub fn error_policy(obj: Arc<RelayMailSes>, err: &Error, _ctx: Arc<Context>) -> Action {
    tracing::warn!(name = %obj.name_any(), error = %err, "reconcile failed, requeueing");
    Action::requeue(Duration::from_secs(30))
}
