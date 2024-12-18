use async_trait::async_trait;

use super::EmbedderError;

#[async_trait]
pub trait Embedder: Send + Sync {
    fn lazy_initialization(
        &self
    ) -> Result<Arc<Mutex<SentenceEmbeddingsModel>>, Box<dyn Error + Send + Sync>>;
    fn initialization(&self) -> Result<SentenceEmbeddingsModel, Box<dyn Error + Send + Sync>>;

    async fn generate_embeddings(&self, text: &str) -> Result<Vec<f32>, EmbedderError>;
}
