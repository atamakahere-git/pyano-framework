pub trait VectorStore {
    fn store(&self, key: &str, vector: Vec<f32>) -> Result<()>;
    fn query(&self, vector: Vec<f32>, limit: usize) -> Result<Vec<Match>>;
}
