use async_trait::async_trait;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use super::process::ModelProcess;
use super::{ ModelConfig, ModelInfo, ModelStatus };
use super::error::{ ModelError, Result };

#[async_trait]
pub trait ModelManager {
    async fn load_model(&self, config: ModelConfig) -> Result<()>;
    async fn unload_model(&self, name: &str) -> Result<()>;
    async fn get_model_status(&self, name: &str) -> Result<ModelStatus>;
    async fn list_models(&self) -> Result<Vec<ModelInfo>>;
}

pub struct ModelManagerImpl {
    models: Arc<RwLock<HashMap<String, ModelProcess>>>,
}

impl ModelManagerImpl {
    pub fn new() -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ModelManager for ModelManagerImpl {
    async fn load_model(&self, config: ModelConfig) -> Result<()> {
        let mut models = self.models.write().await;
        if let Some(process) = models.get(&config.name) {
            if process.status == ModelStatus::Running {
                return Err(ModelError::ModelAlreadyLoaded(config.name));
            }
        }

        let mut process = ModelProcess::new(config);
        process.start().await?;

        models.insert(process.config.name.clone(), process);
        Ok(())
    }

    async fn unload_model(&self, name: &str) -> Result<()> {
        let mut models = self.models.write().await;

        if let Some(process) = models.get_mut(name) {
            process.stop().await?;
            Ok(())
        } else {
            Err(ModelError::ModelNotFound(name.to_string()))
        }
    }

    async fn get_model_status(&self, name: &str) -> Result<ModelStatus> {
        let models = self.models.read().await;

        if let Some(process) = models.get(name) {
            Ok(process.status.clone())
        } else {
            Err(ModelError::ModelNotFound(name.to_string()))
        }
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let models = self.models.read().await;

        Ok(
            models
                .values()
                .map(|process| ModelInfo {
                    name: process.config.name.clone(),
                    model_type: process.config.model_type.clone(),
                    status: process.status.clone(),
                    last_used: process.last_used,
                    server_port: process.config.adapter_config.server_port,
                })
                .collect()
        )
    }
}
