use resources::crd::Mcrouter;
use roperator::prelude::{Error, K8sType, SyncRequest, SyncResponse};
use roperator::serde_json::Value;
use serde_json::json;
use tokio::time::Duration;

pub const OPERATOR_NAME: &str = "mcrouter-operator";

pub static PARENT_TYPE: &K8sType = &K8sType {
    api_version: "example.mcrouter.com/v1",
    kind: "Mcrouter",
    plural_kind: "mcrouters",
};

pub fn handle_sync(request: &SyncRequest) -> Result<SyncResponse, Error> {
    log::info!("Got sync request: {:?}", request);
    let status = json!({
        "message": get_current_status_message(request),
        "phase": "Running",
    });
    let children = get_desired_children(request)?;
    Ok(SyncResponse {
        status,
        children,
        resync: None,
    })
}

pub fn handle_error(request: &SyncRequest, err: Error) -> (Value, Option<Duration>) {
    log::error!("Failed to process request: {:?}\nCause: {:?}", request, err);

    let status = json!({
        "message": err.to_string(),
        "phase": "Error",
    });

    (status, None)
}

fn get_current_status_message(request: &SyncRequest) -> String {
    let pod = request.children().of_type(("v1", "Pod")).first();
    pod.and_then(|p| p.pointer("/status/message").and_then(Value::as_str))
        .unwrap_or("Waiting for Pod to be initialized")
        .to_owned()
}

fn get_desired_children(request: &SyncRequest) -> Result<Vec<Value>, Error> {
    let cr: Mcrouter = request.deserialize_parent()?;
    cr.validate()?;

    let name = cr.metadata.name.as_ref().expect("name");
    let namespace = cr.metadata.namespace.as_ref().expect("namespace");
    let root_service_name = format!("{}-mcrouter-root", name);
    let shard_service_name = |i| format!("{}-mcrouter-shard-{}", name, i);

    let shard_names = (0..cr.spec.num_shards)
        .map(shard_service_name)
        .collect::<Vec<_>>();
    let root_deployment = serde_json::to_value(
        resources::children::root::RootDeployment {
            name: &root_service_name,
            namespace,
            shard_names,
            mcrouter_pool_size: cr.spec.mcrouter_pool_size,
            mcrouter_image: &cr.spec.mcrouter_image,
            mcrouter_port: cr.spec.mcrouter_port,
        }
        .definition(),
    )?;

    let root_service = serde_json::to_value(
        resources::children::root::RootService {
            name: &root_service_name,
            namespace,
            port: cr.spec.mcrouter_port,
        }
        .definition(),
    )?;

    let mut resources = vec![root_deployment, root_service];

    for shard_idx in 0..cr.spec.num_shards {
        let name = shard_service_name(shard_idx);
        let service = serde_json::to_value(
            resources::children::shard::ShardService {
                shard_idx,
                name: &name,
                namespace,
                mcrouter_port: cr.spec.mcrouter_port,
            }
            .definition(),
        )?;

        let stateful_set = serde_json::to_value(
            resources::children::shard::ShardStatefulSet {
                shard_idx,
                name: &name,
                namespace,
                num_replicas: cr.spec.num_replicas,
                mcrouter_image: &cr.spec.mcrouter_image,
                memcached_image: &cr.spec.memcached_image,
                mcrouter_port: cr.spec.mcrouter_port,
            }
            .definition(),
        )?;

        resources.push(service);
        resources.push(stateful_set);
    }

    Ok(resources)
}
