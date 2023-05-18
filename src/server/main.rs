use tonic::transport::Server;

use models::embed::embed_proto::embedder_server::EmbedderServer;
use models::embed::DefaultEmbedder;

pub mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let embedder = DefaultEmbedder::default();

    println!("embedding server listening on {}", addr);

    Server::builder()
        .add_service(EmbedderServer::new(embedder))
        .serve(addr)
        .await?;

    Ok(())
}
