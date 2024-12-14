use crate::llm::llm_builder::LLM;
use std::error::Error as StdError;
use std::pin::Pin;
use super::agent_trait::AgentTrait;
use tokio_stream::StreamExt;

pub struct Agent {
    pub(crate) system_prompt: Option<String>,
    pub(crate) user_prompt: Option<String>,
    pub(crate) stream: Option<bool>,
    pub(crate) llm: Option<LLM>,
    pub(crate) name: Option<String>,
}

impl AgentTrait for Agent {
    fn system_prompt(&self) -> Option<&String> {
        self.system_prompt.as_ref()
    }

    fn user_prompt(&self) -> Option<&String> {
        self.user_prompt.as_ref()
    }

    fn stream(&self) -> bool {
        self.stream.unwrap_or(false)
    }

    fn llm(&self) -> Option<&LLM> {
        self.llm.as_ref()
    }

    fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    fn invoke(
        &self
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<(), Box<dyn StdError + Send + Sync>>> +
                Send +
                '_
        >
    > {
        Box::pin(async move {
            let llm = self.llm.as_ref().expect("LLM is required");
            let system_prompt = self.system_prompt.as_ref().expect("System prompt is missing");
            let user_prompt = self.user_prompt.as_ref().expect("User prompt is missing");
            let stream = self.stream.unwrap_or(false);

            if stream {
                let mut response_stream = llm.response_stream(user_prompt, system_prompt).await?;
                while let Some(response) = response_stream.next().await {
                    match response {
                        Ok(bytes) =>
                            println!("Streamed response: {}", String::from_utf8_lossy(&bytes)),
                        Err(e) => eprintln!("Error streaming response: {}", e),
                    }
                }
            } else {
                let response = llm.response(user_prompt, system_prompt).await?;
                println!("Response: {}", response);
            }

            Ok(())
        })
    }
}
