use std::net::SocketAddr;

use embed::configuration::get_config;
use embed::telemetry::*;
use embed::Application;
use once_cell::sync::Lazy;
use tonic::transport::server::Router;

pub struct TestApp {
    pub address: SocketAddr,
}

pub async fn run_test_app() -> TestApp {
    Lazy::force(&TRACING);

    // Isolate test configuration.
    let configuration = {
        let mut c = get_config().expect("failed to get config");
        c.server.port = 0; // randomly assign to any free port.
        c
    };

    let app = Application::build(configuration.server).expect("failed to build application");
    let addr = app.address;

    let _ = tokio::spawn(run_until_stopped(app.router, addr));

    TestApp { address: addr }
}

async fn run_until_stopped(
    router: Router,
    address: SocketAddr,
) -> Result<(), tonic::transport::Error> {
    router.serve(address).await
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        init_tracing(
            subscriber_name.into(),
            default_filter_level.into(),
            false,
            std::io::stdout,
        );
    } else {
        init_tracing(
            subscriber_name.into(),
            default_filter_level.into(),
            false,
            std::io::sink,
        );
    };
});
