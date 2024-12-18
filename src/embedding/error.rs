use thiserror::Error;
use rust_bert::RustBertError; // Ensure you have this import

#[derive(Error, Debug)]
pub enum EmbedderError {
    #[error("Initialization failed: {0}")] InitializationFailed(String),

    #[error("Embedding generation failed: {0}")] EmbeddingGenerationFailed(String),

    #[error("I/O error: {0}")] IoError(#[from] std::io::Error),

    #[error("Request error: {0}")] RequestError(#[from] reqwest::Error),

    #[error("Rust Bert error: {0}")] RustBertError(#[from] RustBertError),

    #[error("Tokio task error: {0}")] TaskError(#[from] tokio::task::JoinError),

    #[error(transparent)] Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
