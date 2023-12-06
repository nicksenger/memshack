use k8s_openapi::{
    api::core::v1::{Container, Pod, PodSpec},
    apimachinery::pkg::apis::meta::v1::OwnerReference,
};
use serde_json::json;
use thiserror::Error;

use kube::api::{Api, ObjectMeta, Patch, PatchParams, Resource};
use kube::runtime::controller::Action;
use kube::runtime::finalizer::{finalizer, Event as Finalizer};
use kube::Client;
use std::sync::Arc;
use tokio::time::Duration;

use resources::crd::{Mcrouter, McrouterStatus};

pub static WORKLOAD_FINALIZER: &str = "workload.example.dev";

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create Pod: {0}")]
    PodCreationFailed(#[source] kube::Error),
    #[error("MissingObjectKey: {0}")]
    MissingObjectKey(&'static str),
    #[error("Finalizer Error: {0}")]
    // NB: awkward type because finalizer::Error embeds the reconciler error (which is this)
    // so boxing this error to break cycles
    FinalizerError(#[source] Box<kube::runtime::finalizer::Error<Error>>),
}

pub struct Data {
    pub client: Client,
}

/// an error handler that will be called when the reconciler fails with access to both the
/// object that caused the failure and the actual error
pub fn error_policy(obj: Arc<Mcrouter>, error: &Error, _ctx: Arc<Data>) -> Action {
    println!("reconcile failed internal error: {:?}", error);
    Action::requeue(Duration::from_secs(60))
}

fn create_pod(name: &str, namespace: &str, oref: OwnerReference) -> Pod {
    Pod {
        metadata: ObjectMeta {
            name: Some(name.to_owned()),
            namespace: Some(namespace.to_owned()),
            owner_references: Some(vec![oref]),
            ..Default::default()
        },
        spec: Some(PodSpec {
            containers: vec![Container {
                name: name.to_owned(),
                image: Some("busybox".to_owned()),
                command: Some(vec!["sleep".to_owned()]),
                args: Some(vec!["infinity".to_owned()]),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub async fn reconcile(workload: Arc<Mcrouter>, ctx: Arc<Data>) -> Result<Action, Error> {
    let client = &ctx.client;

    let namespace = workload
        .metadata
        .namespace
        .as_ref()
        .ok_or_else(|| Error::MissingObjectKey(".metadata.namespace"))
        .unwrap();

    let name = workload
        .metadata
        .name
        .as_ref()
        .ok_or_else(|| Error::MissingObjectKey(".metadata.names"))
        .unwrap();

    let oref = workload.controller_owner_ref(&()).unwrap();

    let pods: Api<Pod> = Api::namespaced(client.clone(), namespace);
    let workloads: Api<Mcrouter> = Api::namespaced(client.clone(), namespace);

    let current_workloads = workload
        .status
        .clone()
        .unwrap_or_else(|| McrouterStatus::default())
        .pods
        .len();

    finalizer(&workloads, WORKLOAD_FINALIZER, workload, |event| async {
        match event {
            Finalizer::Cleanup(workload) => {
                println!(
                    "Finalizing Workload: {} ...!",
                    workload.meta().name.clone().unwrap()
                );
                Ok(Action::await_change())
            }
            _ => Ok(Action::await_change()),
        }
    })
    .await
    .map_err(|e| Error::FinalizerError(Box::new(e)));
    Ok(Action::requeue(Duration::from_secs(300)))
}
