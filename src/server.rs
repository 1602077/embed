use tonic::{transport::Server, Request, Response, Status};

use embed::embedder_client::EmbedderClient;
use embed::embedder_server::Embedder;
use embed::{EmbedRequest, EmbedResponse};

pub mod embed {
    tonic::include_proto!("embed");
}

#[derive(Default, Clone)]
pub struct DefaultEmbedder {}

#[tonic::async_trait]
impl Embedder for DefaultEmbedder {
    async fn embed(
        &self,
        request: Request<EmbedRequest>,
    ) -> Result<Response<EmbedResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = embed::EmbedResponse {
            body: format!("Hello {}!", request.into_inner().body),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let embedder = DefaultEmbedder::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(EmbedderClient::new(embedder))
        .serve(addr)
        .await?;

    Ok(())
}
