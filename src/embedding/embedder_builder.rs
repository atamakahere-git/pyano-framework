/// The builder that allows configuring which model to use, which texts to embed,
/// and then either returns a configured `DefaultEmbedder` or directly generates embeddings.
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

    /// Directly build an embedder and generate embeddings for the texts added so far.
    /// Returns a Vec of embeddings (one Vec<f32> per input text).
    pub async fn embed_all(self) -> Result<Vec<Vec<f32>>, EmbedderError> {
        let embedder = self.build_embedder();
        let mut results = Vec::new();
        for txt in self.texts {
            let embedding = embedder.generate_embeddings(&txt).await?;
            results.push(embedding);
        }
        Ok(results)
    }
}
