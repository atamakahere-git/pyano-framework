use thiserror::Error; // For custom error handling

#[derive(Error, Debug)]
pub enum LLMError {
    #[error("Server unavailable: {0}")] ServerUnavailable(String),
    #[error("Request failed: {0}")] RequestFailed(String),
    #[error("Unexpected error: {0}")] Unexpected(String),
}
