// Make sure vec0 libraries are installed in the system or the path of the executable.
// To run this example execute: cargo run --example vector_store_sqlite_vec --features sqlite-vec
// Download the libraries from https://github.com/asg017/sqlite-vec
use serde_json::Value;
use std::collections::HashMap;
use pyano::{
    schemas::document::Document,
    vectorstore::{ sqlite_vec::StoreBuilder, VecStoreOptions, VectorStore },
};
use pyano::embedding::{
    embedding_models::{ EmbeddingModels, TextEmbeddingModels },
    embedder_builder::EmbeddingBuilder,
};

use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Embedder
    let model = EmbeddingModels::Text(TextEmbeddingModels::MiniLMV6);
    // Create an embedding builder with the chosen model
    let embedder = EmbeddingBuilder::new(model).build_embedder().await?;

    // Initialize the Sqlite Vector Store
    let store = StoreBuilder::new()
        .embedder(embedder)
        .db_name("micro_app")
        .table("documents")
        .build().await
        .unwrap();

    // Initialize the tables in the database. This is required to be done only once.
    store.initialize().await.unwrap();

    // First document's metadata
    let mut metadata1 = HashMap::new();
    metadata1.insert("author".to_string(), Value::String("Alice".to_string()));
    metadata1.insert("category".to_string(), Value::String("Science".to_string()));
    metadata1.insert("published_year".to_string(), Value::Number((2023).into()));

    // Create the first document (similar to the second document)
    let document1 = Document::new(
        "Quantum mechanics explains how particles behave at the subatomic level."
    ).with_metadata(metadata1);

    // Second document's metadata
    let mut metadata2 = HashMap::new();
    metadata2.insert("author".to_string(), Value::String("Bob".to_string()));
    metadata2.insert("category".to_string(), Value::String("Science".to_string()));
    metadata2.insert("published_year".to_string(), Value::Number((2024).into()));

    // Create the second document (similar to the first document)
    let document2 = Document::new(
        "Quantum mechanics provides a framework for understanding particle interactions at microscopic scales."
    ).with_metadata(metadata2);

    // Third document's metadata
    let mut metadata3 = HashMap::new();
    metadata3.insert("author".to_string(), Value::String("Charlie".to_string()));
    metadata3.insert("category".to_string(), Value::String("History".to_string()));
    metadata3.insert("published_year".to_string(), Value::Number((2020).into()));

    // Create the third document (completely different content)
    let document3 = Document::new(
        "The French Revolution was a period of radical social and political change in France from 1789 to 1799."
    ).with_metadata(metadata3);

    store
        .add_documents(&vec![document1, document2, document3], &VecStoreOptions::default()).await
        .unwrap();

    // Ask for user input
    print!("Query> ");
    std::io::stdout().flush().unwrap();
    let mut query = String::new();
    std::io::stdin().read_line(&mut query).unwrap();

    let results = store.similarity_search(&query, 2, &VecStoreOptions::default()).await.unwrap();

    if results.is_empty() {
        println!("No results found.");
    } else {
        results.iter().for_each(|r| {
            println!("Document: {}", r.page_content);
        });
    }

    Ok(())
}

// #[cfg(not(feature = "sqlite-vec"))]
// fn main() {
//     println!("This example requires the 'sqlite-vec' feature to be enabled.");
//     println!("Please run the command as follows:");
//     println!("cargo run --example vector_store_sqlite_vec --features sqlite-vec");
// }
