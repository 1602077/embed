use tonic::transport::server::Router;
use tonic::transport::Server;
use tracing::info;

use crate::configuration::ServerSettings;
use crate::models::embed::embed_proto::embedder_server::EmbedderServer;
use crate::models::EmbedAdapter;

pub struct Application {
    pub address: String,
    pub router: Router,
}

impl Application {
    pub fn build(config: ServerSettings) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", config.address, config.port);

        info!(message = "building application", config = ?config);

        let embedder = EmbedAdapter::build(config.embedder)?;

        let router =
            Server::builder().add_service(EmbedderServer::new(embedder));

        Ok(Self { address, router })
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.address.parse().unwrap();
        info!(message = "embedding server started");
        self.router.serve(addr).await?;
        Ok(())
    }
}
