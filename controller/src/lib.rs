use resources::crd::Memshack;
use roperator::prelude::{Error, K8sType, SyncRequest, SyncResponse};
use roperator::serde_json::Value;
use serde_json::json;
use tokio::time::Duration;

pub const OPERATOR_NAME: &str = "memshack-operator";

pub static PARENT_TYPE: &K8sType = &K8sType {
    api_version: "example.memshack.com/v1",
    kind: "Memshack",
    plural_kind: "memshacks",
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
    let cr: Memshack = request.deserialize_parent()?;
    cr.validate()?;

    let name = cr.metadata.name.as_ref().expect("name");
    let namespace = cr.metadata.namespace.as_ref().expect("namespace");
    let gateway_name = format!("{}-memshack", name);
    let shard_name = |i| format!("{}-memshack-shard-{}", name, i);

    let shard_names = (0..cr.spec.num_shards).map(shard_name).collect::<Vec<_>>();
    let gateway_deployment = serde_json::to_value(
        resources::children::gateway::GatewayDeployment {
            name: &gateway_name,
            namespace,
            shard_names,
            mcrouter_pool_size: cr.spec.mcrouter_pool_size,
            mcrouter_image: &cr.spec.mcrouter_image,
            mcrouter_port: cr.spec.mcrouter_port,
        }
        .definition(),
    )?;

    let gateway_service = serde_json::to_value(
        resources::children::gateway::GatewayService {
            name: &gateway_name,
            namespace,
            port: cr.spec.mcrouter_port,
        }
        .definition(),
    )?;

    let mut resources = vec![gateway_deployment, gateway_service];

    for shard_idx in 0..cr.spec.num_shards {
        let name = shard_name(shard_idx);
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
