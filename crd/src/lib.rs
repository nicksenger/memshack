use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use kube::CustomResource;

pub static FINALIZER: &str = "mcrouter.example.com";

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct McrouterStatus {
    pub pods: Vec<String>,
}

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "example.com", version = "v1", kind = "Mcrouter", namespaced)]
#[kube(status = "McrouterStatus")]
#[kube(scale = r#"{"specReplicasPath":".spec.replicas", "statusReplicasPath":".status.replicas"}"#)]
pub struct McrouterSpec {
    pub replicas: usize,
}
