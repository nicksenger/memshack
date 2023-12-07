use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use kube::CustomResource;

use crate::MEMCACHED_PORT;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "example.mcrouter.com",
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
    #[serde(default = "default_mcrouter_pool_size")]
    pub mcrouter_pool_size: usize,
    #[serde(default = "default_num_shards")]
    pub num_shards: usize,
    #[serde(default = "default_num_replicas")]
    pub num_replicas: usize,
}

impl Mcrouter {
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if self.spec.mcrouter_port == MEMCACHED_PORT {
            anyhow::bail!("mcrouter_port must not be {}!", MEMCACHED_PORT);
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct McrouterStatus {
    pub pods: Vec<String>,
}

fn default_mcrouter_image() -> String {
    // TODO: push an image to hub for this
    "jamescarr/mcrouter:latest".to_string()
}

fn default_memcached_image() -> String {
    // TODO: push an image to hub for this
    "memcached:1.6-alpine".to_string()
}

fn default_mcrouter_port() -> usize {
    5000
}

fn default_mcrouter_pool_size() -> usize {
    2
}

fn default_num_shards() -> usize {
    3
}

fn default_num_replicas() -> usize {
    2
}
