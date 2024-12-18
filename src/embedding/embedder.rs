/// The default embedder that actually handles downloading the model and generating embeddings.
pub struct DefaultEmbedder {
    model_path: PathBuf,
    base_url: String,
    files: Vec<String>,
}

impl DefaultEmbedder {
    pub fn new(model_path: &Path, base_url: &str, files: Vec<String>) -> Self {
        Self {
            model_path: model_path.to_path_buf(),
            base_url: base_url.to_string(),
            files,
        }
    }

    fn download_file(
        &self,
        client: &Client,
        file_url: &str,
        save_path: &Path
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let url = format!("{}{}", self.base_url, file_url);
        let file_path = save_path.join(file_url);

        if let Some(parent_dir) = file_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        if !file_path.exists() {
            let response = client.get(&url).send()?;
            if !response.status().is_success() {
                return Err(
                    format!(
                        "Failed to download file: {}. Status: {}",
                        file_url,
                        response.status()
                    ).into()
                );
            }

            let content = response.bytes()?;
            let mut dest_file = File::create(&file_path)?;
            let mut content_reader = Cursor::new(content);
            std::io::copy(&mut content_reader, &mut dest_file)?;
        }

        Ok(())
    }

    fn download_files(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let client = Client::new();
        for file in &self.files {
            self.download_file(&client, file, &self.model_path)?;
        }
        Ok(())
    }

    fn ensure_model_files(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !self.model_path.exists() {
            fs::create_dir_all(&self.model_path)?;
        }
        self.download_files()?;
        Ok(())
    }
}

#[async_trait]
impl Embedder for DefaultEmbedder {
    fn lazy_initialization(
        &self
    ) -> Result<Arc<Mutex<SentenceEmbeddingsModel>>, Box<dyn Error + Send + Sync>> {
        // We need a static Lazy that depends on self.model_path, which is tricky.
        // Instead, we can just do a lazy init per instance.
        // If you need a truly global lazy init, you'd have to restructure this approach.
        static INIT_ONCE: Lazy<()> = Lazy::new(|| {});
        let model_path = self.model_path.clone();

        // Ensure the files are present before model initialization.
        self.ensure_model_files()?;

        let model_builder = SentenceEmbeddingsBuilder::local(model_path)
            .with_device(Device::Cpu)
            .create_model();
        model_builder.map(|m| Arc::new(Mutex::new(m))).map_err(|e| e.into())
    }

    fn initialization(&self) -> Result<SentenceEmbeddingsModel, Box<dyn Error + Send + Sync>> {
        self.ensure_model_files()?;
        SentenceEmbeddingsBuilder::local(self.model_path.clone())
            .with_device(Device::Cpu)
            .create_model()
            .map_err(|e| e.into())
    }

    async fn generate_embeddings(&self, text: &str) -> Result<Vec<f32>, EmbedderError> {
        let text_owned = text.to_string();
        let lazy_model = self.lazy_initialization()?;
        let embedding = tokio::task::spawn_blocking(move || {
            let model_guard = lazy_model.lock().unwrap();
            let embeddings = model_guard.encode(&[&text_owned])?;
            Ok::<Vec<f32>, Box<dyn Error + Send + Sync>>(embeddings[0].clone())
        }).await??;

        Ok(embedding)
    }
}
