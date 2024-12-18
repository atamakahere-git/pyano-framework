/// The builder that allows configuring which model to use, which texts to embed,
/// and then either returns a configured `DefaultEmbedder` or directly generates embeddings.

use super::embedding_models::EmbeddingModels;
use super::embedder::DefaultEmbedder;
use dirs;

pub struct EmbeddingBuilder {
    model: EmbeddingModels,
    texts: Vec<String>,
}

impl EmbeddingBuilder {
    /// Create a new builder with a chosen model.
    pub fn new(model: EmbeddingModels) -> Self {
        Self {
            model,
            texts: Vec::new(),
        }
    }

    /// Add a text string to be embedded later.
    pub fn with_text(mut self, text: &str) -> Self {
        self.texts.push(text.to_string());
        self
    }

    /// Add multiple texts at once.
    pub fn with_texts<I: IntoIterator<Item = String>>(mut self, texts: I) -> Self {
        self.texts.extend(texts);
        self
    }

    /// Build a `DefaultEmbedder` instance. This ensures the model files are present
    /// and sets everything up. Note that this does not generate embeddings yet.
    pub fn build_embedder(&self) -> DefaultEmbedder {
        let model_path = dirs
            ::home_dir()
            .expect("Unable to get home directory")
            .join(self.model.model_path());

        DefaultEmbedder::new(
            &model_path,
            self.model.base_url(),
            self.model
                .required_files()
                .iter()
                .map(|f| f.to_string())
                .collect()
        )
    }
}
