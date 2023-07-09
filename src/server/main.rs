use embed::configuration::get_config;
use embed::server::Application;
use embed::telemetry::init_tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config().expect("failed to load config");

    init_tracing(
        config.server.log.name.clone(),
        config.server.log.level.clone(),
        config.server.log.json,
        std::io::stdout,
    );

    Application::build(config.server)
        .expect("failed to build applicationg")
        .run()
        .await
}
