use pyano::model::{ ModelManager, ModelManagerServer };
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and start the server
    let manager = Arc::new(ModelManager::new());
    let server = ModelManagerServer::new(manager);
    server.run("127.0.0.1:8090").await?;
    Ok(())
}
