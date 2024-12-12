use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Model not found: {0}")] ModelNotFound(String),

    #[error("Model already loaded: {0}")] ModelAlreadyLoaded(String),

    #[error("Failed to start model process: {0}")] ProcessError(String),

    #[error("Invalid configuration: {0}")] ConfigError(String),

    #[error("HTTP error: {0}")] HttpError(#[from] reqwest::Error),

    #[error("IO error: {0}")] IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ModelError>;
