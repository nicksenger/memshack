use k8s_openapi::api::{
    apps::v1::{StatefulSet, StatefulSetSpec},
    core::v1::{
        Container, ContainerPort, PodSpec, PodTemplateSpec, Service, ServicePort, ServiceSpec,
    },
};
use kube::core::ObjectMeta;
use serde_json::json;

use crate::MEMCACHED_PORT;

pub struct ShardStatefulSet<'a> {
    pub shard_idx: usize,
    pub namespace: &'a str,
    pub name: &'a str,
    pub num_replicas: usize,
    pub mcrouter_image: &'a str,
    pub memcached_image: &'a str,
    pub mcrouter_port: usize,
}

impl<'a> ShardStatefulSet<'a> {
    pub fn definition(self) -> StatefulSet {
        StatefulSet {
            metadata: ObjectMeta {
                name: Some(self.name.to_string()),
                namespace: Some(self.namespace.to_string()),
                ..Default::default()
            },
            spec: Some(StatefulSetSpec {
                replicas: Some(self.num_replicas as i32),
                template: PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        name: Some(format!("{}-pod", self.name)),
                        namespace: Some(self.namespace.to_string()),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        containers: vec![
                            Container {
                                name: "mcrouter".to_string(),
                                image: Some(self.mcrouter_image.to_string()),
                                ports: Some(vec![ContainerPort {
                                    container_port: self.mcrouter_port as i32,
                                    protocol: Some("TCP".to_string()),
                                    ..Default::default()
                                }]),
                                command: Some(
                                    [
                                        "mcrouter".to_string(),
                                        format!("--config-str='{}'", self.config_json()),
                                        "-p".to_string(),
                                        self.mcrouter_port.to_string(),
                                    ]
                                    .into_iter()
                                    .collect(),
                                ),
                                ..Default::default()
                            },
                            Container {
                                name: "memcached".to_string(),
                                image: Some(self.mcrouter_image.to_string()),
                                ports: Some(vec![ContainerPort {
                                    container_port: MEMCACHED_PORT as i32,
                                    protocol: Some("TCP".to_string()),
                                    ..Default::default()
                                }]),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    })
                },
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    fn config_json(&self) -> String {
        json!( {
          "pools": {
             "A": {
                "servers": (0..self.num_replicas)
                    .map(|replica_idx| {
                        format!(
                            "{}-{}.{}.{}.svc.cluster.local:{}",
                            self.name,
                            replica_idx,
                            self.name,
                            self.namespace,
                            MEMCACHED_PORT
                        )
                    })
                    .collect::<Vec<_>>()
             }
          },
          "route": {
            "type": "OperationSelectorRoute",
            "operation_policies": {
              "add": "AllSyncRoute|Pool|A",
              "delete": "AllSyncRoute|Pool|A",
              "get": "LatestRoute|Pool|A",
              "set": "AllSyncRoute|Pool|A"
            }
          }
        })
        .to_string()
    }
}

pub struct ShardService<'a> {
    pub shard_idx: usize,
    pub namespace: &'a str,
    pub name: &'a str,
    pub mcrouter_port: usize,
}

impl<'a> ShardService<'a> {
    pub fn definition(self) -> Service {
        Service {
            metadata: ObjectMeta {
                name: Some(self.name.to_string()),
                namespace: Some(self.namespace.to_string()),
                ..Default::default()
            },
            spec: Some(ServiceSpec {
                type_: Some("ClusterIP".to_string()),
                selector: Some(
                    [("app.kubernetes.io/name".to_string(), self.name.to_string())]
                        .into_iter()
                        .collect(),
                ),
                ports: Some(vec![ServicePort {
                    port: self.mcrouter_port as i32,
                    target_port: Some(
                        k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(
                            self.mcrouter_port as i32,
                        ),
                    ),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
