use resources::crd::Mcrouter;
use roperator::prelude::{Error, K8sType, SyncRequest, SyncResponse};
use roperator::serde_json::Value;
use serde_json::json;
use tokio::time::Duration;

pub const OPERATOR_NAME: &str = "mcrouter-operator";

/// a `K8sType` with basic info about our parent CRD
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

/// Returns the json value that should be set on the parent EchoServer
fn get_current_status_message(request: &SyncRequest) -> String {
    let pod = request.children().of_type(("v1", "Pod")).first();
    pod.and_then(|p| p.pointer("/status/message").and_then(Value::as_str))
        .unwrap_or("Waiting for Pod to be initialized")
        .to_owned()
}

fn get_desired_children(request: &SyncRequest) -> Result<Vec<Value>, Error> {
    let custom_resource: Mcrouter = request.deserialize_parent()?;

    let name = custom_resource.metadata.name.as_ref().expect("name");
    let namespace = custom_resource
        .metadata
        .namespace
        .as_ref()
        .expect("namespace");
    let root_service_name = format!("{}-mcrouter-root", name);
    let shard_service_name = format!("{}-mcrouter-shard", name);

    Ok(vec![])
}
