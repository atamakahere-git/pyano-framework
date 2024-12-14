// #[derive(Clone, Debug)]
// pub struct AgentPrompt {
//     pub system: String,
//     pub user_template: String,
//     pub output_format: Option<String>,
// }
use crate::llm::llm_builder::LLM;
use std::error::Error as StdError; // Importing the correct trait
use std::pin::Pin;
pub trait AgentTrait: Send + Sync {
    fn system_prompt(&self) -> Option<&String>;
    fn user_prompt(&self) -> Option<&String>;
    fn stream(&self) -> bool;
    fn llm(&self) -> Option<&LLM>;
    fn name(&self) -> Option<&String>;

    fn invoke(
        &self
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<(), Box<dyn StdError + Send + Sync>>> +
                Send +
                '_
        >
    >;
}
