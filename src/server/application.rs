use std::net::SocketAddr;

use tonic::transport::server::Router;
use tonic::transport::Server;
use tracing::info;

use crate::configuration::ServerSettings;
use crate::models::embed::embed_proto::embedder_server::EmbedderServer;
use crate::models::EmbedAdapter;

pub struct Application {
    pub address: SocketAddr,
    pub router: Router,
}

impl Application {
    #[tracing::instrument(name = "build embed server", skip(config))]
    pub fn build(config: ServerSettings) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", config.address, config.port)
            .parse()
            .expect("failed to parse socket address");

        let embedder = EmbedAdapter::build(config.embedder)?;

        let router =
            Server::builder().add_service(EmbedderServer::new(embedder));

        Ok(Self { address, router })
    }

    #[tracing::instrument(name = "run embed server", skip(self))]
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        info!(message = "embedding server started", addr = ?self.address);
        self.router.serve(self.address).await?;
        Ok(())
    }
}
