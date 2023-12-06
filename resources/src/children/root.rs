use k8s_openapi::api::{core::v1::{Service, ServicePort, ServiceSpec}, apps::v1::Deployment};
use kube::core::ObjectMeta;

pub struct RootDeployment {
    pub namespace: String,
    pub name: String,
    pub num_shards: usize,
}

impl RootDeployment {
    pub fn definition() -> Deployment {
        Default::default() // TODO
    }
}

pub struct RootService {
    pub namespace: String,
    pub name: String,
    pub port: i32,
    pub service_name: String,
}

impl RootService {
    pub fn definition(self) -> Service {
        Service {
            metadata: ObjectMeta {
                name: Some(self.name),
                namespace: Some(self.namespace),
                ..Default::default()
            },
            spec: Some(ServiceSpec {
                type_: Some("ClusterIP".to_string()),
                selector: Some(
                    [("app.kubernetes.io/name".to_string(), self.service_name)]
                        .into_iter()
                        .collect(),
                ),
                ports: Some(vec![ServicePort {
                    port: self.port,
                    target_port: Some(
                        k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(self.port),
                    ),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
