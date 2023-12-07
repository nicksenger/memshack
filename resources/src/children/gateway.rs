use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{
            Container, ContainerPort, PodSpec, PodTemplateSpec, Service, ServicePort, ServiceSpec,
        },
    },
    apimachinery::pkg::apis::meta::v1::LabelSelector,
};
use kube::core::ObjectMeta;
use serde_json::json;

pub struct GatewayDeployment<'a> {
    pub namespace: &'a str,
    pub name: &'a str,
    pub mcrouter_pool_size: usize,
    pub mcrouter_image: &'a str,
    pub mcrouter_port: usize,
    pub shard_names: Vec<String>,
}

impl<'a> GatewayDeployment<'a> {
    pub fn definition(self) -> Deployment {
        Deployment {
            metadata: ObjectMeta {
                name: Some(self.name.to_string()),
                namespace: Some(self.namespace.to_string()),
                ..Default::default()
            },
            spec: Some(DeploymentSpec {
                replicas: Some(self.mcrouter_pool_size as i32),
                selector: LabelSelector {
                    match_labels: Some(
                        [(self.name.to_string(), "gateway".to_string())]
                            .into_iter()
                            .collect(),
                    ),
                    ..Default::default()
                },
                template: PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        name: Some(format!("{}-pod", self.name)),
                        namespace: Some(self.namespace.to_string()),
                        labels: Some(
                            [(self.name.to_string(), "gateway".to_string())]
                                .into_iter()
                                .collect(),
                        ),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        containers: vec![Container {
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
                                    format!("--config-str={}", self.config_json()),
                                    "--asynclog-disable".to_string(),
                                    "--async-dir".to_string(),
                                    "/".to_string(),
                                    "-p".to_string(),
                                    self.mcrouter_port.to_string(),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    fn config_json(&self) -> String {
        json!({
          "pools": {
             "A": {
                "servers": self
                    .shard_names
                    .iter()
                    .map(|shard_name| {
                        format!(
                            "{}.{}.svc.cluster.local:{}",
                            shard_name, self.namespace, self.mcrouter_port
                        )
                    })
                    .collect::<Vec<_>>()
             }
          },
          "route": "PoolRoute|A"
        })
        .to_string()
    }
}

pub struct GatewayService<'a> {
    pub namespace: &'a str,
    pub name: &'a str,
    pub port: usize,
}

impl<'a> GatewayService<'a> {
    pub fn definition(self) -> Service {
        Service {
            metadata: ObjectMeta {
                name: Some(self.name.to_string()),
                namespace: Some(self.namespace.to_string()),
                ..Default::default()
            },
            spec: Some(ServiceSpec {
                type_: Some("ClusterIP".to_string()),
                cluster_ip: Some(String::new()),
                selector: Some(
                    [(self.name.to_string(), "gateway".to_string())]
                        .into_iter()
                        .collect(),
                ),
                ports: Some(vec![ServicePort {
                    port: self.port as i32,
                    target_port: Some(
                        k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(
                            self.port as i32,
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
