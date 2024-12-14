use crate::llm::llm_builder::LLM;
use std::error::Error as StdError;
use std::pin::Pin;
use super::agent_trait::AgentTrait;
use tokio_stream::StreamExt;

pub struct Agent {
    system_prompt: Option<String>,
    user_prompt: Option<String>,
    stream: Option<bool>,
    llm: Option<LLM>,
    name: Option<String>,
}

pub struct AgentBuilder {
    system_prompt: Option<String>,
    user_prompt: Option<String>,
    stream: Option<bool>,
    llm: Option<LLM>,
    name: Option<String>,
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self {
            system_prompt: None,
            user_prompt: None,
            stream: Some(false),
            llm: None,
            name: None,
        }
    }

    pub fn with_system_prompt(mut self, system_prompt: String) -> Self {
        self.system_prompt = Some(system_prompt);
        self
    }

    pub fn with_user_prompt(mut self, user_prompt: String) -> Self {
        self.user_prompt = Some(user_prompt);
        self
    }

    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn with_llm(mut self, llm: LLM) -> Self {
        self.llm = Some(llm);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Agent {
        if self.llm.is_none() {
            panic!("LLM must be provided before building the Agent");
        }
        if self.user_prompt.is_none() {
            panic!("User prompt must be provided before building the Agent");
        }
        if self.system_prompt.is_none() {
            panic!("System prompt must be provided before building the Agent");
        }

        Agent {
            system_prompt: self.system_prompt,
            user_prompt: self.user_prompt,
            stream: self.stream,
            llm: self.llm,
            name: self.name,
        }
    }
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
