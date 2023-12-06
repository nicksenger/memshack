use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use kube::CustomResource;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "example.com",
    version = "v1",
    kind = "Mcrouter",
    namespaced,
    status = "McrouterStatus"
)]
pub struct McrouterSpec {
    #[serde(default = "default_mcrouter_image")]
    pub mcrouter_image: String,
    #[serde(default = "default_memcached_image")]
    pub memcached_image: String,
    #[serde(default = "default_mcrouter_port")]
    pub mcrouter_port: usize,
    #[serde(default = "default_memcached_port")]
    pub memcached_port: usize,
    #[serde(default = "default_pool_size")]
    pub memcached_pool_size: usize,
    #[serde(default)]
    pub pool_setup: PoolSetup,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct McrouterStatus {
    pub pods: Vec<String>,
}

#[derive(Default, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum PoolSetup {
    #[default]
    Replicated,
    Sharded,
}

fn default_mcrouter_image() -> String {
    "mcrouter/mcrouter".to_string()
}

fn default_memcached_image() -> String {
    "memcached:1.6-alpine".to_string()
}

fn default_mcrouter_port() -> usize {
    5000
}

fn default_memcached_port() -> usize {
    11211
}

fn default_pool_size() -> usize {
    3
}
