use pyano::{ agent::{ text::{ Summarizer, Analyzer }, Agent }, model::ModelManager };

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize model manager
    // or connect to already running model manager 192.168.1.4:5000
    let model_manager = Arc::new(ModelManager::new());

    // Load a model if not loaded
    model_manager.load_model("llama-7b", Config::default())?;

    // Create text processing pipeline
    let summarizer = Summarizer::new(&model_manager);
    let analyzer = Analyzer::new(&model_manager);

    // Process text

    // Use default prompt
    let summary = summarizer.process("Long text here...")?;

    // Or customize the prompt
    summary = summarizer.with_prompt(AgentPrompt {
        system: "You are a technical documentation summarizer...".to_string(),
        user_template: "Create a technical summary of:\n{input}".to_string(),
        output_format: Some("Key technical points in bullet form".to_string()),
    });

    let analysis = analyzer.process(&summary)?;

    println!("Analysis: {:?}", analysis);
    Ok(())
}
