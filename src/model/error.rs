use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Model not found: {0}")] ModelNotFound(String),

    #[error("Model already loaded: {0}")] ModelAlreadyLoaded(String),

    #[error("Process error: {0}")] ProcessError(String),

    #[error("Configuration error: {0}")] ConfigError(String),

    #[error("Request failed: {0}")] RequestError(#[from] reqwest::Error),

    #[error("Failed to deserialize response: {0}")] DeserializationError(#[from] serde_json::Error),

    #[error("Server error: {0}")] ServerError(String),

    #[error("Invalid model configuration: {0}")] InvalidConfig(String),

    #[error("Memory error: {0}")] MemoryError(String),

    #[error("IO error: {0}")] IoError(#[from] std::io::Error),
}

pub type ModelResult<T> = std::result::Result<T, ModelError>;
