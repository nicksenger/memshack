use controller;

use futures::StreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::api::Api;
use kube::runtime::controller::Controller;
use kube::runtime::watcher;
use kube::Client;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    let wrkld = Api::<crd::Mcrouter>::all(client.clone());
    let podapi = Api::<Pod>::all(client.clone());

    Controller::new(wrkld, watcher::Config::default())
        .owns(podapi, watcher::Config::default())
        .shutdown_on_signal()
        .run(
            controller::reconcile,
            controller::error_policy,
            Arc::new(controller::Data { client }),
        )
        .for_each(|res| async move {
            match res {
                Ok(o) => println!("reconciled {:?}", o),
                Err(e) => println!("reconcile failed: {}", e),
            }
        })
        .await;

    Ok(())
}
