use kube::CustomResourceExt;

fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").expect("manifest directory");

    let crd = serde_yaml::to_string(&resources::crd::Memshack::crd()).expect("CRD yaml");
    let deployment =
        serde_yaml::to_string(&resources::operator::deployment()).expect("Deployment yaml");
    let service_account = serde_yaml::to_string(&resources::operator::service_account())
        .expect("Service Account yaml");
    let role_binding =
        serde_yaml::to_string(&resources::operator::role_binding()).expect("Role Binding yaml");
    let role = serde_yaml::to_string(&resources::operator::role()).expect("Role yaml");

    let yaml = [role, role_binding, service_account, deployment, crd].join("---\n");
    std::fs::write(format!("{dir}/../yaml/operator.yaml"), yaml).expect("write yaml");
}
