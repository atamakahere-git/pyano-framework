use std::process::{ Child, Command };
use chrono::{ DateTime, Utc };
use tokio::sync::oneshot;

use super::{ ModelConfig, ModelStatus };
use super::error::{ ModelError, Result };

pub(crate) struct ModelProcess {
    pub config: ModelConfig,
    pub child: Option<Child>,
    pub status: ModelStatus,
    pub last_used: DateTime<Utc>,
    pub shutdown_signal: Option<oneshot::Sender<()>>,
}

impl ModelProcess {
    pub fn new(config: ModelConfig) -> Self {
        Self {
            config,
            child: None,
            status: ModelStatus::Stopped,
            last_used: Utc::now(),
            shutdown_signal: None,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        if self.status == ModelStatus::Running {
            return Ok(());
        }

        let mut cmd = Command::new("/Users/cj/.pyano/build/bin/llama-server");

        // Configure command based on adapter config
        cmd.arg("-m")
            .arg(&self.config.model_path)
            .arg("--ctx-size")
            .arg(self.config.adapter_config.ctx_size.to_string())
            .arg("--no-mmap");

        if let Some(port) = self.config.adapter_config.server_port {
            cmd.arg("--port").arg(port.to_string());
        }

        // Add extra arguments
        for (key, value) in &self.config.adapter_config.extra_args {
            cmd.arg(format!("--{}", key)).arg(value);
        }

        let child = cmd.spawn().map_err(|e| ModelError::ProcessError(e.to_string()))?;

        self.child = Some(child);
        self.status = ModelStatus::Running;
        self.last_used = Utc::now();

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            child.kill().map_err(|e| ModelError::ProcessError(e.to_string()))?;
            self.status = ModelStatus::Stopped;
        }
        Ok(())
    }
}
