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
    let embedder = EmbeddingBuilder::new(model).build_embedder().await?;
    // Initialize the embedder (downloads model files if not present)
    // Define a batch of texts to embed
    let texts = vec![
        "Hello, world!",
        "This is a sample sentence for embedding.",
        "Another example sentence."
    ];

    let embeddings = embedder.generate_embeddings_with_cache(&texts).await?;

    // Print out the embeddings
    for (i, embedding) in embeddings.iter().enumerate() {
        println!("Embedding {} length: {}", i + 1, embedding.len());
        // Optional: Print first few dimensions of each embedding
        println!("Embedding {} first 5 dimensions: {:?}", i + 1, &embedding[..5]);
    }

    Ok(())
}
