#[derive(Clone, Debug)]
pub struct AgentPrompt {
    pub system: String,
    pub user_template: String,
    pub output_format: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AgentConfig {
    pub name: String,
    pub prompt: AgentPrompt,
    pub model_name: String,
    pub max_tokens: usize,
}

pub trait Agent: Send + Sync {
    /// Get the default prompt for this agent
    fn default_prompt(&self) -> AgentPrompt;

    /// Update the agent's prompt
    fn with_prompt(&mut self, prompt: AgentPrompt) -> &mut Self;

    /// Process input with the current prompt
    fn process(&self, input: &str) -> Result<String>;

    fn get_capabilities(&self) -> Vec<Capability>;
}
