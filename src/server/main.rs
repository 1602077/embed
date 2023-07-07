use embed::configuration::get_config;
use embed::server::startup::Application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config().expect("failed to load config");

    Application::build(config)
        .expect("failed to build application from config")
        .run()
        .await
}
