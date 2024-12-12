use async_trait::async_trait;

use super::{ error::Result, ModelConfig, ModelInfo, ModelManager, ModelStatus };

pub struct ModelManagerClient {
    base_url: String,
    client: reqwest::Client,
}

impl ModelManagerClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl ModelManager for ModelManagerClient {
    async fn load_model(&self, config: ModelConfig) -> Result<()> {
        let url = format!("{}/models/load", self.base_url);
        self.client.post(&url).json(&config).send().await?.error_for_status()?;
        Ok(())
    }

    async fn unload_model(&self, name: &str) -> Result<()> {
        let url = format!("{}/models/unload", self.base_url);
        self.client.post(&url).json(&name).send().await?.error_for_status()?;
        Ok(())
    }

    async fn get_model_status(&self, name: &str) -> Result<ModelStatus> {
        let url = format!("{}/models/status/{}", self.base_url, name);
        let response = self.client.get(&url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let url = format!("{}/models/list", self.base_url);
        let response = self.client.get(&url).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }
}
