use std::error::Error as StdError;

use pyano::{
    llm::{ options::LLMHTTPCallOptions, llm_builder::LLM },
    agent::{ agent_builder::AgentBuilder, agent_trait::AgentTrait },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let prompt_template =
        "
            <|im_start|>system
            {system_prompt}<|im_end|>
            <|im_start|>user
            {user_prompt}<|im_end|>
            <|im_start|>assistant
            ";
    // Create LLMHTTPCallOptions with required configurations
    let options = LLMHTTPCallOptions::new()
        .with_server_url("http://localhost:52555/".to_string())
        .with_prompt_template(prompt_template.to_string())
        .with_temperature(0.7)
        .build();

    // Define the system prompt
    let system_prompt = "You are a great summarizer and your task is to summarize the content";
    // Build the LLM instance
    let llm = LLM::builder().with_options(options).build();

    // Define the user prompt
    let user_prompt =
        "Summarize the following: Rust is a multi-paradigm programming language focusing on performance and safety.";

    // Execute the LLM call with the user prompt

    // Print the response
    let agent = AgentBuilder::new()
        .with_system_prompt(system_prompt.to_string())
        .with_user_prompt(user_prompt.to_string())
        .with_stream(true)
        .with_llm(llm)
        .build();

    if let Err(e) = agent.invoke().await {
        eprintln!("Error during summarization: {}", e);
    }
    Ok(())
}
