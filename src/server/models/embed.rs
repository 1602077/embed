use embed_proto::embedder_server::Embedder;
use embed_proto::{EmbedRequest, EmbedResponse};
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use tonic::{Request, Response, Status};
use tracing::info;

use self::embed_proto::embed_response;
use crate::configuration::{EmbedSettings, EmbedderType};

pub mod embed_proto {
    tonic::include_proto!("embed");
}

#[derive(Debug)]
pub struct EmbedAdapter {}
impl EmbedAdapter {
    #[tracing::instrument(name = "building embedder")]
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

#[derive(Default, Clone, Debug)]
pub struct Transformer {
    pub model: String,
    // pub model: Arc<SentenceEmbeddingsModel>,
}
// impl Transformer {
//     pub fn new(model: String) -> Result<Self, String> {
//         let model_type = ModelType::try_from(model)?.0;
//         let model =
//             SentenceEmbeddingsBuilder::remote(model_type).create_model()?;
//         Ok(Transformer {
//             model: Arc::clone(&model),
//         })
//     }
// }

// TODO: This works although is a rather hacky, as:
//  - models are pulled live for each request (very bad)
//  - get this issue (Cannot drop a runtime in a context where blocking is not allowed) which is
//  resolved which a release build.
#[tonic::async_trait]
impl Embedder for Transformer {
    #[tracing::instrument(name = "embedding request recieved")]
    async fn embed(
        &self,
        request: Request<EmbedRequest>,
    ) -> Result<Response<EmbedResponse>, Status> {
        let model_type =
            ModelType::try_from(self.model.clone()).expect("failed").0;
        let model = SentenceEmbeddingsBuilder::remote(model_type)
            .create_model()
            .unwrap();

        let body = &request.get_ref().body;

        let vector_embedding =
            &model.encode(&[body]).expect("failed to embed")[0];

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
            "all-mini-lm-l6" => {
                Ok(ModelType(SentenceEmbeddingsModelType::AllMiniLmL6V2))
            }
            "all-mini-lm-l12" => {
                Ok(ModelType(SentenceEmbeddingsModelType::AllMiniLmL12V2))
            }
            "all-distilroberta" => {
                Ok(ModelType(SentenceEmbeddingsModelType::AllDistilrobertaV1))
            }
            "bert-base-nli-mean-tokens" => Ok(ModelType(
                SentenceEmbeddingsModelType::BertBaseNliMeanTokens,
            )),
            "distil-use-base-multilingual-cased" => Ok(ModelType(
                SentenceEmbeddingsModelType::DistiluseBaseMultilingualCased,
            )),
            "paraphrase-albert-small-v2" => Ok(ModelType(
                SentenceEmbeddingsModelType::ParaphraseAlbertSmallV2,
            )),
            "sentence-t5" => {
                Ok(ModelType(SentenceEmbeddingsModelType::SentenceT5Base))
            }
            other => {
                Err(format!("{} is not a supported embedder type.", other))
            }
        }
    }
}
