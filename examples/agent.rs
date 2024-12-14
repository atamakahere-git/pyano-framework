use std::sync::Arc;

// use pyano::{
//     agent::{ AgentExecutor, ConversationalAgentBuilder },
//     chain::{ options::ChainCallOptions, Chain },
//     llm::openai::{ OpenAI, OpenAIModel },

//     memory::SimpleMemory,
//     prompt_args,
//     tools::CommandExecutor,
// };

use pyano::llm::options::LLMHTTPCallOptions;

#[tokio::main]
async fn main() {
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
    let llm = LLM::builder()
        .with_options(options)
        .with_system_prompt(system_prompt.to_string())
        .build();

    // Define the user prompt
    let user_prompt =
        "Summarize the following: Rust is a multi-paradigm programming language focusing on performance and safety.";

    // Execute the LLM call with the user prompt
    let response = llm.response_stream(user_prompt).await?;

    // Print the response
    println!("LLM Response: {}", response);

    Ok(())

    // let llm = OpenAI::default().with_model(OpenAIModel::Gpt4Turbo);
    // let memory = SimpleMemory::new();
    // let command_executor = CommandExecutor::default();
    // let agent = ConversationalAgentBuilder::new()
    //     .tools(&[Arc::new(command_executor)])
    //     .options(ChainCallOptions::new().with_max_tokens(1000))
    //     .build(llm)
    //     .unwrap();

    // let executor = AgentExecutor::from_agent(agent).with_memory(memory.into());

    // let input_variables =
    //     prompt_args! {
    //     "input" => "What is the name of the current dir",
    // };

    // match executor.invoke(input_variables).await {
    //     Ok(result) => {
    //         println!("Result: {:?}", result);
    //     }
    //     Err(e) => panic!("Error invoking LLMChain: {:?}", e),
    // }
}
