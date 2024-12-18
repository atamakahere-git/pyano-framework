use pyano::embedding::{
    embedding_models::{ EmbeddingModels, TextEmbeddingModels },
    embedder_builder::EmbeddingBuilder,
    embedder_trait::Embedder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Choose the MiniLM v6 text embedding model
    let model = EmbeddingModels::Text(TextEmbeddingModels::MiniLMV6);

    // Create an embedding builder with the chosen model
    let embedder = EmbeddingBuilder::new(model)
        .with_text("Hello, world!")
        .with_text("This is a sample sentence for embedding.")
        .build_embedder();

    // Initialize the embedder (downloads model files if not present)
    embedder.initialize().await?;

    // Generate embeddings for the texts
    let embedding1 = embedder.generate_embeddings_with_cache("Hello, world!").await?;
    let embedding2 = embedder.generate_embeddings_with_cache(
        "This is a sample sentence for embedding."
    ).await?;

    // Print out the embeddings
    println!("Embedding 1 length: {}", embedding1.len());
    println!("Embedding 2 length: {}", embedding2.len());

    // Optional: Print first few dimensions of each embedding
    println!("Embedding 1 first 5 dimensions: {:?}", &embedding1[..5]);
    println!("Embedding 2 first 5 dimensions: {:?}", &embedding2[..5]);

    Ok(())
}
