use crate::model::ModelManager;
use crate::types::common::Message;

pub struct SummarizerAgent {
    name: String,
    model_manager: ModelManager,
}

impl SummarizerAgent {
    pub fn new(name: String, model_manager: ModelManager) -> Self {
        SummarizerAgent {
            name,
            model_manager,
        }
    }

    pub fn greet(&self) -> Message {
        Message {
            content: format!("Hello from Summary Agent: {}", self.name),
        }
    }

    pub fn process_with_model(&self) -> Vec<Message> {
        vec![self.greet(), self.model_manager.hello_world(), Message {
            content: "Processing complete!".to_string(),
        }]
    }
}

// pub struct SummarizerAgent {
//     config: AgentConfig,
//     model_manager: Arc<ModelManager>,
// }

// impl SummarizerAgent {
//     pub fn new(model_manager: Arc<ModelManager>) -> Self {
//         Self {
//             config: AgentConfig {
//                 name: "summarizer".to_string(),
//                 prompt: AgentPrompt {
//                     system: "You are a precise text summarizer...".to_string(),
//                     user_template: "Summarize the following text:\n{input}".to_string(),
//                     output_format: Some("Brief summary in 2-3 sentences.".to_string()),
//                 },
//                 max_tokens: 150,
//             },
//             model_manager,
//         }
//     }
// }

// impl Agent for SummarizerAgent {
//     fn default_prompt(&self) -> AgentPrompt {
//         self.config.prompt.clone()
//     }

//     fn with_prompt(&mut self, prompt: AgentPrompt) -> &mut Self {
//         self.config.prompt = prompt;
//         self
//     }

//     fn process(&self, input: &str) -> Result<String> {
//         // Implementation details...
//     }
// }
