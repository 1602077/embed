use tonic::transport::server::Router;
use tonic::transport::Server;

use crate::configuration::Settings;
use crate::models::embed::embed_proto::embedder_server::EmbedderServer;
use crate::models::EmbedAdapter;

pub struct Application {
    pub address: String,
    pub router: Router,
}

impl Application {
    pub fn build(config: Settings) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", config.address, config.port);

        let embedder = EmbedAdapter::build(config.embedder)?;

        let router =
            Server::builder().add_service(EmbedderServer::new(embedder));

        Ok(Self { address, router })
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.address.parse().unwrap();
        println!("embedding server listening on {}", addr);
        self.router.serve(addr).await?;
        Ok(())
    }
}
