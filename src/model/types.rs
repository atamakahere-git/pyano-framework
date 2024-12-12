use serde::{ Deserialize, Serialize };
use std::path::PathBuf;
use std::collections::HashMap;
use chrono::{ DateTime, Utc };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub model_path: PathBuf,
    pub model_type: ModelType,
    pub adapter_config: AdapterConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelType {
    Text,
    Voice,
    Vision,
    #[serde(untagged)] Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    pub server_port: Option<u16>,
    pub ctx_size: usize,
    pub gpu_layers: i32,
    pub batch_size: usize,
    pub extra_args: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub model_type: ModelType,
    pub status: ModelStatus,
    pub last_used: DateTime<Utc>,
    pub server_port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelStatus {
    Loading,
    Running,
    Stopped,
    Error(String),
}
