use async_trait::async_trait;
use std::error::Error;

enum EmbedderError {
    InitializationFailed(String),
    EmbeddingGenerationFailed(String),
}
