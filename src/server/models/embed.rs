use tonic::{Request, Response, Status};

use embed_proto::embedder_server::Embedder;
use embed_proto::{EmbedRequest, EmbedResponse};

pub mod embed_proto {
    tonic::include_proto!("embed");
}

#[derive(Default)]
pub struct DefaultEmbedder {}

#[tonic::async_trait]
impl Embedder for DefaultEmbedder {
    async fn embed(
        &self,
        request: Request<EmbedRequest>,
    ) -> Result<Response<EmbedResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = EmbedResponse {
            body: format!("Hello {}!", request.into_inner().body),
        };

        Ok(Response::new(reply))
    }
}
