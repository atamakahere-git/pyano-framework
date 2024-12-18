use std::process::{ Child, Command };
use std::thread::sleep;
use std::time::Duration;
use chrono::{ DateTime, Utc };
use log::{ info, error };
use tokio::sync::oneshot;

use super::{ ModelConfig, ModelStatus };
use super::error::{ ModelError, ModelResult };

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

    pub async fn start(&mut self) -> ModelResult<()> {
        if self.status == ModelStatus::Running {
            return Ok(());
        }

        self.status = ModelStatus::Loading;

        let mut cmd = Command::new("/Users/cj/.pyano/build/bin/llama-server");

        // Configure command based on adapter config
        cmd.arg("-m")
            .arg(&self.config.model_path)
            .arg("--ctx-size")
            .arg(self.config.server_config.ctx_size.to_string());

        if let Some(port) = self.config.server_config.port {
            cmd.arg("--port").arg(port.to_string());
        }

        if let Some(threads) = self.config.server_config.num_threads {
            cmd.arg("--threads").arg(threads.to_string());
        }

        if self.config.server_config.gpu_layers > 0 {
            cmd.arg("--n-gpu-layers").arg(self.config.server_config.gpu_layers.to_string());
        }

        if !self.config.server_config.use_mmap {
            cmd.arg("--no-mmap");
        }

        // Add batch size
        cmd.arg("--batch-size").arg(self.config.server_config.batch_size.to_string());

        // Add extra arguments
        for (key, value) in &self.config.server_config.extra_args {
            cmd.arg(format!("--{}", key)).arg(value);
        }

        match cmd.spawn() {
            Ok(child) => {
                sleep(Duration::from_secs(10));
                self.child = Some(child);
                self.status = ModelStatus::Running;
                self.last_used = Utc::now();
                Ok(())
            }
            Err(e) => {
                self.status = ModelStatus::Error(e.to_string());
                Err(ModelError::ProcessError(e.to_string()))
            }
        }
    }
    pub async fn stop(&mut self) -> ModelResult<()> {
        if let Some(mut child) = self.child.take() {
            let pid = child.id();

            // Try graceful shutdown first
            if let Err(e) = child.kill() {
                error!("Failed to kill process gracefully: {}", e);
                // Force kill as backup
                unsafe {
                    libc::kill(pid as i32, libc::SIGKILL);
                }
            }
            sleep(Duration::from_secs(5));
            // Wait for process to exit with timeout
            let _ = tokio::time::timeout(std::time::Duration::from_secs(1), async {
                let mut child = child;
                let _ = child.wait();
            }).await;

            // Force kill again if still running
            unsafe {
                libc::kill(pid as i32, libc::SIGKILL);
            }
        }

        self.status = ModelStatus::Stopped;
        self.child = None;

        Ok(())
    }
}
