use k8s_openapi::api::apps::{self, v1::DeploymentSpec};
use k8s_openapi::api::core::v1::{
    Container, ContainerPort, PodSpec, PodTemplateSpec, ServiceAccount,
};
use k8s_openapi::api::rbac::v1::{PolicyRule, Role, RoleBinding, RoleRef, Subject};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use kube::core::ObjectMeta;

pub fn deployment() -> apps::v1::Deployment {
    apps::v1::Deployment {
        metadata: ObjectMeta {
            name: Some("mcrouter-operator".to_string()),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(1),
            selector: LabelSelector {
                match_labels: Some(
                    [("name".to_string(), "mcrouter-operator".to_string())]
                        .into_iter()
                        .collect(),
                ),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    name: Some("mcrouter-operator".to_string()),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    service_account_name: Some("mcrouter-operator".to_string()),
                    containers: vec![Container {
                        name: "operator".to_string(),
                        image: Some("mcrouter-operator:latest".to_string()),
                        image_pull_policy: Some("Never".to_string()),
                        ports: Some(vec![ContainerPort {
                            container_port: 3000,
                            name: Some("https".to_string()),
                            protocol: Some("TCP".to_string()),
                            ..Default::default()
                        }]),
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

pub fn service_account() -> ServiceAccount {
    ServiceAccount {
        metadata: ObjectMeta {
            name: Some("mcrouter-operator".to_string()),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn role_binding() -> RoleBinding {
    RoleBinding {
        metadata: ObjectMeta {
            name: Some("mcrouter-operator".to_string()),
            ..Default::default()
        },
        subjects: Some(vec![Subject {
            kind: "ServiceAccount".to_string(),
            name: "mcrouter-operator".to_string(),
            ..Default::default()
        }]),
        role_ref: RoleRef {
            kind: "Role".to_string(),
            name: "mcrouter-operator".to_string(),
            api_group: "rbac.authorization.k8s.io".to_string(),
        }
    }
}

pub fn role() -> Role {
    Role {
        metadata: ObjectMeta {
            name: Some("mcrouter-operator".to_string()),
            ..Default::default()
        },
        rules: Some(vec![
            PolicyRule {
                api_groups: Some(vec!["".to_string()]),
                resources: Some(
                    [
                        "pods",
                        "services",
                        "endpoints",
                        "persistentvolumeclaims",
                        "events",
                        "configmaps",
                        "secrets",
                    ]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
                ),
                verbs: vec!["*".to_string()],
                ..Default::default()
            },
            PolicyRule {
                api_groups: Some(vec!["apps".to_string()]),
                resources: Some(
                    ["deployments", "daemonsets", "replicasets", "statefulsets"]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ),
                verbs: vec!["*".to_string()],
                ..Default::default()
            },
            PolicyRule {
                api_groups: Some(vec!["apps".to_string()]),
                resource_names: Some(
                    ["mcrouter-operator"]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ),
                resources: Some(vec!["deployments/finalizers".to_string()]),
                verbs: vec!["update".to_string()],
                ..Default::default()
            },
            PolicyRule {
                api_groups: Some(vec!["memcached.example.com".to_string()]),
                resources: Some(["*"].into_iter().map(ToString::to_string).collect()),
                verbs: vec!["*".to_string()],
                ..Default::default()
            },
            PolicyRule {
                api_groups: Some(vec!["mcrouter.example.com".to_string()]),
                resources: Some(
                    ["*", "mcrouters"]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ),
                verbs: vec!["*".to_string()],
                ..Default::default()
            },
        ])
    }
}
