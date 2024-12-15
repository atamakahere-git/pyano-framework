use std::error::Error as StdError;
use serde_json::{ json, Value };
use log::{ info, error };
use pyano::{
    llm::{
        options::LLMHTTPCallOptions,
        llm_builder::LLM,
        stream_processing::llamacpp_process_stream,
    },
    agent::{ agent_builder::AgentBuilder, agent_trait::AgentTrait },
    tools::DuckDuckGoSearchResults,
    tools::WebScrapper,
    tools::Tool,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    env_logger::init();
    let ddg = DuckDuckGoSearchResults::default().with_max_results(3);
    let query = json!({"query": "Give me some facts about Peru?"}).to_string();
    let mut links: Vec<String> = Vec::new(); // Declare links variable outside the block
    // Fetch links from the DuckDuckGoSearchResults tool
    let response_value = match ddg.json_call(&query).await {
        Ok(value) => value,
        Err(e) => {
            error!("Error performing search: {}", e);
            Value::Null
        }
    };

    if !response_value.is_null() {
        links = DuckDuckGoSearchResults::extract_links_from_results(response_value);
    } else {
        error!("No valid response received.");
    }

    print!("{:?}", links);

    let scraper = WebScrapper::new();

    let mut aggregated_content = String::new();
    let urls = json!({"urls": links}).to_string();

    match scraper.json_call(&urls).await {
        Ok(result) => {
            // Parse the JSON result
            if let Some(results) = result["results"].as_array() {
                for item in results {
                    if let Some(content) = item.get("content").and_then(|c| c.as_str()) {
                        aggregated_content.push_str(content);
                        aggregated_content.push(' '); // Add a space between contents
                    } else if let Some(error) = item.get("error").and_then(|e| e.as_str()) {
                        eprintln!("Error for URL: {}", error);
                    }
                }
            } else {
                eprintln!("Unexpected output format: {:?}", result);
            }
        }
        Err(e) => {
            eprintln!("Error occurred: {}", e);
        }
    }

    println!("Aggregated Content: {}", aggregated_content.trim());

    // let prompt_template =
    //     "
    //         <|im_start|>system
    //         {system_prompt}<|im_end|>
    //         <|im_start|>user
    //         {user_prompt}<|im_end|>
    //         <|im_start|>assistant
    //         ";

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

    // Define the system prompt
    let system_prompt = "You are a great summarizer and your task is to summarize the content";
    // Build the LLM instance
    let llm = LLM::builder()
        .with_options(options)
        .with_process_response(|stream| Box::pin(llamacpp_process_stream(stream)))
        .build();

    // Define the user prompt
    let user_prompt = aggregated_content;

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
