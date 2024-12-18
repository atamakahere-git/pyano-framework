use std::path::{ PathBuf, Path };
use tokio::fs::{ self, File };
use tokio::io::AsyncWriteExt;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::{ Mutex, OnceCell };
use super::error::EmbedderError;
use async_trait::async_trait;
use super::embedder_trait::Embedder;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder,
    SentenceEmbeddingsModel,
};
use tokio::task;
use tch::Device;

pub struct DefaultEmbedder {
    model_path: PathBuf,
    base_url: String,
    files: Vec<String>,
    // Optional cached model using OnceCell for thread-safe lazy initialization
    cached_model: OnceCell<Arc<Mutex<SentenceEmbeddingsModel>>>,
}

impl DefaultEmbedder {
    pub fn new(model_path: &Path, base_url: &str, files: Vec<String>) -> Self {
        Self {
            model_path: model_path.to_path_buf(),
            base_url: base_url.to_string(),
            files,
            cached_model: OnceCell::new(),
        }
    }

    /// Downloads a single file if it doesn't exist.
    async fn download_file(
        &self,
        client: &Client,
        file_url: &str,
        save_path: &Path
    ) -> Result<(), EmbedderError> {
        let url = format!("{}{}", self.base_url, file_url);
        let file_path = save_path.join(file_url);

        if let Some(parent_dir) = file_path.parent() {
            fs::create_dir_all(parent_dir).await?;
        }

        if !file_path.exists() {
            let response = client.get(&url).send().await?;
            if !response.status().is_success() {
                return Err(
                    EmbedderError::InitializationFailed(
                        format!(
                            "Failed to download file: {}. Status: {}",
                            file_url,
                            response.status()
                        )
                    )
                );
            }

            let content = response.bytes().await?;
            let mut dest_file = File::create(&file_path).await?;
            dest_file.write_all(&content).await?;
        }

        Ok(())
    }

    /// Downloads all required files.
    async fn download_files(&self) -> Result<(), EmbedderError> {
        let client = Client::new();
        for file in &self.files {
            self.download_file(&client, file, &self.model_path).await?;
        }
        Ok(())
    }

    /// Ensures that all model files are present.
    async fn prepare_model_files(&self) -> Result<(), EmbedderError> {
        if !self.model_path.exists() {
            fs::create_dir_all(&self.model_path).await?;
        }

        self.download_files().await?;

        Ok(())
    }

    /// Create model with improved error handling
    async fn create_model(&self) -> Result<Arc<Mutex<SentenceEmbeddingsModel>>, EmbedderError> {
        self.prepare_model_files().await?;

        let model = SentenceEmbeddingsBuilder::local(&self.model_path)
            .with_device(Device::Cpu)
            .create_model()?;

        Ok(Arc::new(Mutex::new(model)))
    }
}

#[async_trait]
impl Embedder for DefaultEmbedder {
    /// Initializes and caches the model for future use.
    async fn initialize(&self) -> Result<(), EmbedderError> {
        self.cached_model
            .get_or_try_init(|| async { self.create_model().await }).await
            .map(|_| ())
            .map_err(|e| EmbedderError::InitializationFailed(e.to_string()))
    }

    async fn generate_embeddings_with_cache(&self, text: &str) -> Result<Vec<f32>, EmbedderError> {
        // Ensure model is initialized, uses cached version
        let model = self.cached_model.get_or_try_init(|| async {
            self.create_model().await.map_err(|e|
                EmbedderError::InitializationFailed(e.to_string())
            )
        }).await?;

        let text_owned = text.to_owned();
        let model_clone = Arc::clone(model);

        // Generate embeddings in a blocking task
        let embedding = task
            ::spawn_blocking(move || {
                let model_guard = futures::executor::block_on(model_clone.lock());
                model_guard.encode(&[&text_owned])
            }).await
            .map_err(|e| EmbedderError::TaskError(e))?
            .map_err(|e| EmbedderError::RustBertError(e))?;

        embedding
            .into_iter()
            .next()
            .ok_or(EmbedderError::EmbeddingGenerationFailed("No embedding generated".to_string()))
    }
    async fn generate_embeddings_on_demand(&self, text: &str) -> Result<Vec<f32>, EmbedderError> {
        // Create a new model instance for this specific call
        let model = self.create_model().await?;

        let text_owned = text.to_owned();
        let model_clone = Arc::clone(&model);

        // Generate embeddings in a blocking task
        let embedding = task
            ::spawn_blocking(move || {
                let model_guard = futures::executor::block_on(model_clone.lock());
                model_guard.encode(&[&text_owned])
            }).await
            .map_err(|e| EmbedderError::TaskError(e))?
            .map_err(|e| EmbedderError::RustBertError(e))?;

        embedding
            .into_iter()
            .next()
            .ok_or(EmbedderError::EmbeddingGenerationFailed("No embedding generated".to_string()))
    }
}
