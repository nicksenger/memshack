use roperator::config::{ChildConfig, OperatorConfig};
use roperator::k8s_types::apps::v1::{Deployment, StatefulSet};
use roperator::k8s_types::core::v1::Service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "roperator=info,warn");
    }
    env_logger::init();

    let operator_config = OperatorConfig::new(controller::OPERATOR_NAME, controller::PARENT_TYPE)
        .with_child(Service, ChildConfig::replace())
        .with_child(Deployment, ChildConfig::replace())
        .with_child(StatefulSet, ChildConfig::replace());

    let err = roperator::runner::run_operator(
        operator_config,
        (controller::handle_sync, controller::handle_error),
    );

    log::error!("Error running operator: {}", err);
    std::process::exit(1);
}
