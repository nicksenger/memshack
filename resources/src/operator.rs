use k8s_openapi::api::apps::{self, v1::DeploymentSpec};
use k8s_openapi::api::core::v1::{
    Container, ContainerPort, PodSpec, PodTemplateSpec, ServiceAccount,
};
use k8s_openapi::api::rbac::v1::{ClusterRole, ClusterRoleBinding, PolicyRule, RoleRef, Subject};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use kube::core::ObjectMeta;

pub fn deployment() -> apps::v1::Deployment {
    apps::v1::Deployment {
        metadata: ObjectMeta {
            name: Some("memshack-operator".to_string()),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(2),
            selector: LabelSelector {
                match_labels: Some(
                    [("name".to_string(), "memshack-operator".to_string())]
                        .into_iter()
                        .collect(),
                ),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    name: Some("memshack-operator".to_string()),
                    labels: Some(
                        [("name".to_string(), "memshack-operator".to_string())]
                            .into_iter()
                            .collect(),
                    ),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    service_account_name: Some("memshack-operator".to_string()),
                    containers: vec![Container {
                        name: "operator".to_string(),
                        image: Some("memshack-operator:latest".to_string()),
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
            name: Some("memshack-operator".to_string()),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn role_binding() -> ClusterRoleBinding {
    ClusterRoleBinding {
        metadata: ObjectMeta {
            name: Some("memshack-operator".to_string()),
            ..Default::default()
        },
        subjects: Some(vec![Subject {
            kind: "ServiceAccount".to_string(),
            name: "memshack-operator".to_string(),
            namespace: Some("default".to_string()),
            ..Default::default()
        }]),
        role_ref: RoleRef {
            kind: "ClusterRole".to_string(),
            name: "memshack-operator".to_string(),
            api_group: "rbac.authorization.k8s.io".to_string(),
        },
    }
}

pub fn role() -> ClusterRole {
    ClusterRole {
        metadata: ObjectMeta {
            name: Some("memshack-operator".to_string()),
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
                    ["memshack-operator"]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ),
                resources: Some(vec!["deployments/finalizers".to_string()]),
                verbs: vec!["update".to_string()],
                ..Default::default()
            },
            PolicyRule {
                api_groups: Some(vec!["example.memcached.com".to_string()]),
                resources: Some(["*"].into_iter().map(ToString::to_string).collect()),
                verbs: vec!["*".to_string()],
                ..Default::default()
            },
            PolicyRule {
                api_groups: Some(vec!["example.memshack.com".to_string()]),
                resources: Some(
                    ["*", "memshacks"]
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                ),
                verbs: vec!["*".to_string()],
                ..Default::default()
            },
        ]),
        ..Default::default()
    }
}
