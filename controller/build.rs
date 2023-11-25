use kube::CustomResourceExt;

fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").expect("manifest directory");
    let crd = serde_yaml::to_string(&crd::Mcrouter::crd()).expect("CRD yaml");
    std::fs::write(&format!("{dir}/../yaml/crd.yaml"), crd).expect("write CRD");
}
