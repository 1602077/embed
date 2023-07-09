use embed_proto::embedder_server::Embedder;
use embed_proto::{EmbedRequest, EmbedResponse};
use tonic::{Request, Response, Status};
use tracing::info;

use crate::configuration::{EmbedSettings, EmbedderType};

pub mod embed_proto {
    tonic::include_proto!("embed");
}

#[derive(Debug)]
pub struct EmbedAdapter {}
impl EmbedAdapter {
    pub fn build(
        config: EmbedSettings,
    ) -> Result<impl Embedder, std::io::Error> {
        info!(
            message = "building embedder",
            implementation = config.r#impl.as_str(),
            model = config.model
        );
        match config.r#impl {
            EmbedderType::Transformer => Ok(Transformer {
                model: config.model,
            }),
        }
    }
}

#[derive(Default)]
pub struct Transformer {
    pub model: String,
}

#[tonic::async_trait]
impl Embedder for Transformer {
    async fn embed(
        &self,
        request: Request<EmbedRequest>,
    ) -> Result<Response<EmbedResponse>, Status> {
        let reply = EmbedResponse {
            id: request.into_inner().id,
            body: "".into(), // body: format!("Hello {}!", request.into_inner().body),
        };

        Ok(Response::new(reply))
    }
}
