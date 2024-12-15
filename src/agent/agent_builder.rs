use crate::llm::llm_builder::LLM;
use super::agent::Agent;
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
