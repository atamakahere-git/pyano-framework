# Usage

```rust
// Local usage
let manager = Arc::new(ModelManagerImpl::new());

let  llama_extra_args = HashMap::new();

let config = ModelConfig  {
    name: "coder".to_string(),
    model_path: PathBuf::from("/Users/cj/.pyano/models/base-coder.gguf"),
    model_type: ModelType::Text,
    adapter_config: AdapterConfig {
        batch_size: 512,
        ctx_size: 8192,
        gpu_layers: -1,
        server_port: Some(5006),
        extra_args: llama_extra_args
    }
};
// use below load model with your api or other provider usage.
if let Err(e) = model_manager.load_model(config).await {
    eprintln!("Failed to load model: {}", e);
}

// Remote usage
let manager = ModelManagerClient::new("http://localhost:8080");
```

### Local Flow

```
Application -> ModelManager -> Model Process -> Adapter APIs
```

### Remote Flow

```
Application -> ModelManagerClient -> HTTP -> ModelManagerServer -> ModelManager -> Model Process -> Adapter APIs
```

### Server binary provides

```rust
let manager = Arc::new(ModelManagerImpl::new(config));
let server = ModelManagerServer::new(manager);
server.run("127.0.0.1:8080").await?;
```
