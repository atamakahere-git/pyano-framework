use std::error::Error as StdError;
use pyano::{
    llm::{
        options::LLMHTTPCallOptions,
        llm_builder::LLM,
        stream_processing::llamacpp_process_stream,
    },
    agent::agent_builder::AgentBuilder,
    chain::sequential_chain::{ Chain, ExecutionRecord },
};

use std::sync::{ Arc, Mutex };

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let prompt_template =
        "
        <|begin_of_text|><|start_header_id|>system<|end_header_id|>
            Cutting Knowledge Date: December 2023
            Today Date: 26 Jul 2024
        {system_prompt}<|eot_id|><|start_header_id|>user<|end_header_id|>
        {user_prompt}<|eot_id|><|start_header_id|>assistant<|end_header_id|>
    ";
    // Create LLMHTTPCallOptions with required configurations
    let options = LLMHTTPCallOptions::new()
        .with_server_url("http://localhost:52555".to_string())
        .with_prompt_template(prompt_template.to_string())
        .with_temperature(0.7)
        .build();

    // Build the LLM instance
    let llm = LLM::builder()
        .with_options(options)
        .with_process_response(|stream| Box::pin(llamacpp_process_stream(stream)))
        .build();

    // Define system prompts for each agent
    let system_prompt_1 = "You are an excellent content generator.";
    let system_prompt_2 = "You are a great analyzer of generated content.";
    let system_prompt_3 = "You are a summarizer for analyzed content.";

    // Define user prompts for each agent
    let user_prompt_1 = "Generate content on the topic - Future of AI agents";
    let user_prompt_2 = "Analyze the generated content.";
    let user_prompt_3 = "Summarize the analysis.";

    // Create agentsc
    let agent_1 = Arc::new(
        Mutex::new(
            AgentBuilder::new()
                .with_name(String::from("Content Generator Agent"))
                .with_system_prompt(system_prompt_1.to_string())
                .with_user_prompt(user_prompt_1.to_string())
                .with_stream(true)
                .with_llm(llm.clone())
                .build()
        )
    );

    let agent_2 = Arc::new(
        Mutex::new(
            AgentBuilder::new()
                .with_name(String::from("Analyzer Agent"))
                .with_system_prompt(system_prompt_2.to_string())
                .with_user_prompt(user_prompt_2.to_string())
                .with_stream(true)
                .with_llm(llm.clone())
                .build()
        )
    );

    let agent_3 = Arc::new(
        Mutex::new(
            AgentBuilder::new()
                .with_name(String::from("Summarizer Agent"))
                .with_system_prompt(system_prompt_3.to_string())
                .with_user_prompt(user_prompt_3.to_string())
                .with_stream(true)
                .with_llm(llm.clone())
                .build()
        )
    );

    // Create a chain and add agents
    let mut chain = Chain::new().add_agent(agent_1).add_agent(agent_2).add_agent(agent_3);

    // Run the chain
    if let Err(e) = chain.run().await {
        eprintln!("Error executing chain: {}", e);
    }

    // Access the memory logs
    let logs = chain.memory_logs();
    for log in logs {
        println!("Agent: {}, Timestamp: {:?}", log.agent_name, log.timestamp);
    }
    Ok(())
}
