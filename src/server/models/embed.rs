use std::sync::{Arc, Mutex};

use embed_proto::embedder_server::Embedder;
use embed_proto::{EmbedRequest, EmbedResponse};
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};
use tonic::{Request, Response, Status};
use tracing::info;

use self::embed_proto::embed_response;
use crate::configuration::{EmbedSettings, EmbedderType};

pub mod embed_proto {
    tonic::include_proto!("embed");
}

// TODO: anyhow error handling and try to remove lazy unwraps.

#[derive(Debug)]
pub struct EmbedAdapter {}
impl EmbedAdapter {
    #[tracing::instrument(name = "building embedder")]
    pub fn build(config: EmbedSettings) -> Result<impl Embedder, std::io::Error> {
        info!(
            message = "building embedder",
            implementation = config.r#impl.as_str(),
            model = config.model
        );
        match config.r#impl {
            EmbedderType::Transformer => {
                let model = ModelType::try_from(config.model).unwrap().0;
                let model = SentenceEmbeddingsBuilder::remote(model)
                    .create_model()
                    .unwrap();
                Ok(Transformer {
                    model: Arc::new(Mutex::new(model)),
                })
            }
        }
    }
}

pub struct Transformer {
    pub model: Arc<Mutex<SentenceEmbeddingsModel>>,
}

#[tonic::async_trait]
impl Embedder for Transformer {
    #[tracing::instrument(name = "embedding request recieved", skip(self))]
    async fn embed(
        &self,
        request: Request<EmbedRequest>,
    ) -> Result<Response<EmbedResponse>, Status> {
        let body = &request.get_ref().body;

        let vector_embedding = &self
            .model
            .lock()
            .unwrap()
            .encode(&[body])
            .expect("failed to embed")[0];

        let embedding = embed_response::Embedding {
            vector: vector_embedding.to_vec(),
        };

        Ok(Response::new(EmbedResponse {
            id: request.into_inner().id,
            embedding: embedding.into(),
        }))
    }
}

pub struct ModelType(SentenceEmbeddingsModelType);

impl TryFrom<String> for ModelType {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        // TODO: This feels overly verbose, there must be a nicer way of
        // extending the rust-bert enum.
        match value.to_lowercase().as_str() {
            "all-mini-lm-l6" => Ok(ModelType(SentenceEmbeddingsModelType::AllMiniLmL6V2)),
            "all-mini-lm-l12" => Ok(ModelType(SentenceEmbeddingsModelType::AllMiniLmL12V2)),
            "all-distilroberta" => Ok(ModelType(SentenceEmbeddingsModelType::AllDistilrobertaV1)),
            "bert-base-nli-mean-tokens" => Ok(ModelType(
                SentenceEmbeddingsModelType::BertBaseNliMeanTokens,
            )),
            "distil-use-base-multilingual-cased" => Ok(ModelType(
                SentenceEmbeddingsModelType::DistiluseBaseMultilingualCased,
            )),
            "paraphrase-albert-small-v2" => Ok(ModelType(
                SentenceEmbeddingsModelType::ParaphraseAlbertSmallV2,
            )),
            "sentence-t5" => Ok(ModelType(SentenceEmbeddingsModelType::SentenceT5Base)),
            other => Err(format!("{} is not a supported embedder type.", other)),
        }
    }
}
