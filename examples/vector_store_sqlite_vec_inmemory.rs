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
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Embedder
    let model = EmbeddingModels::Text(TextEmbeddingModels::MiniLMV6);
    // Create an embedding builder with the chosen model
    let embedder = EmbeddingBuilder::new(model).build_embedder().await?;

    // Initialize the Sqlite Vector Store
    let store = StoreBuilder::new()
        .embedder(embedder.clone())
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

    // Third document's metadata
    let mut metadata4 = HashMap::new();
    metadata4.insert("author".to_string(), Value::String("Mason".to_string()));
    metadata4.insert("category".to_string(), Value::String("Physics".to_string()));
    metadata4.insert("published_year".to_string(), Value::Number((2020).into()));

    // Create the third document (completely different content)
    let document4 = Document::new(
        "Quantum mechanics explains the fundamental principles governing particles at the atomic level."
    ).with_metadata(metadata4);

    // Third document's metadata
    let mut metadata5 = HashMap::new();
    metadata5.insert("author".to_string(), Value::String("Alex".to_string()));
    metadata5.insert("category".to_string(), Value::String("Science".to_string()));
    metadata5.insert("published_year".to_string(), Value::Number((2020).into()));

    // Create the third document (completely different content)
    let document5 = Document::new(
        "Quantum theory explores the principles that determine the behavior of particles at tiny scales."
    ).with_metadata(metadata5);

    // Third document's metadata
    let mut metadata6 = HashMap::new();
    metadata6.insert("author".to_string(), Value::String("Duke".to_string()));
    metadata6.insert("category".to_string(), Value::String("Quantum mechanics".to_string()));
    metadata6.insert("published_year".to_string(), Value::Number((2020).into()));

    // Create the third document (completely different content)
    let document6 = Document::new(
        "Quantum physics investigates the nature of particles and waves in the subatomic realm."
    ).with_metadata(metadata6);

    store
        .add_documents(
            &vec![document1, document2, document3, document4, document5, document6],
            &VecStoreOptions::default()
        ).await
        .unwrap();

    // // Ask for user input
    // print!("Query> ");
    // std::io::stdout().flush().unwrap();
    let query = String::from(
        "Quantum physics describes the behavior of matter and energy on a microscopic scale."
    );
    // std::io::stdin().read_line(&mut query).unwrap();

    // let options = VecStoreOptions::new();

    let options = VecStoreOptions::new()
        .with_filters(json!({"category": "Science"}))
        .with_embedder(embedder);

    let results = store.similarity_search(&query, 2, &options).await.unwrap();

    if results.is_empty() {
        println!("No results found.");
    } else {
        results.iter().for_each(|r| {
            println!("Document: {}", r.page_content);
        });
        // println!("{:?}", results);
    }

    Ok(())
}

// #[cfg(not(feature = "sqlite-vec"))]
// fn main() {
//     println!("This example requires the 'sqlite-vec' feature to be enabled.");
//     println!("Please run the command as follows:");
//     println!("cargo run --example vector_store_sqlite_vec --features sqlite-vec");
// }
