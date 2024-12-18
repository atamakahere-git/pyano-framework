use async_trait::async_trait;
use super::error::EmbedderError;

// #[async_trait]
// pub trait Embedder: Send + Sync {
//     async fn lazy_initialization(
//         &self
//     ) -> Result<Arc<Mutex<SentenceEmbeddingsModel>>, Box<dyn Error + Send + Sync>>;
//     async fn initialization(&self) -> Result<SentenceEmbeddingsModel, Box<dyn Error + Send + Sync>>;

//     async fn generate_embeddings(&self, text: &str) -> Result<Vec<f32>, EmbedderError>;
// }

#[async_trait]
pub trait Embedder: Send + Sync {
    /// Initializes the model and caches it for future use.
    async fn initialize(&self) -> Result<(), EmbedderError>;

    /// Generates embeddings using the cached model.
    async fn generate_embeddings_with_cache(&self, text: &str) -> Result<Vec<f32>, EmbedderError>;

    /// Generates embeddings by loading the model on demand.
    async fn generate_embeddings_on_demand(&self, text: &str) -> Result<Vec<f32>, EmbedderError>;
}
