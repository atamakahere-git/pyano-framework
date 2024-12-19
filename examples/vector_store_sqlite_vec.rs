// Make sure vec0 libraries are installed in the system or the path of the executable.
// To run this example execute: cargo run --example vector_store_sqlite_vec --features sqlite-vec
// Download the libraries from https://github.com/asg017/sqlite-vec

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

    let database_url = std::env::var("DATABASE_URL").unwrap_or("sqlite::memory:".to_string());

    // Initialize the Sqlite Vector Store
    let store = StoreBuilder::new()
        .embedder(embedder)
        .connection_url(database_url)
        .table("documents")
        .vector_dimensions(1536)
        .build().await
        .unwrap();

    // Initialize the tables in the database. This is required to be done only once.
    store.initialize().await.unwrap();

    // Add documents to the database
    let doc1 = Document::new(
        "langchain-rust is a port of the langchain python library to rust and was written in 2024."
    );
    let doc2 = Document::new(
        "langchaingo is a port of the langchain python library to go language and was written in 2023."
    );
    let doc3 = Document::new(
        "Capital of United States of America (USA) is Washington D.C. and the capital of France is Paris."
    );
    let doc4 = Document::new("Capital of France is Paris.");

    store.add_documents(&vec![doc1, doc2, doc3, doc4], &VecStoreOptions::default()).await.unwrap();

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
