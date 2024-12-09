use crate::types::common::Message;

pub struct ModelManager {
    name: String,
}

impl ModelManager {
    pub fn new(name: String) -> Self {
        ModelManager { name }
    }

    pub fn hello_world(&self) -> Message {
        Message {
            content: format!("Hello from Model Manager: {}", self.name),
        }
    }
}

// pub trait ModelManager {
// fn load_model(&self, config: ModelConfig) -> Result<()>;
// fn unload_model(&self, name: &str) -> Result<()>;
// fn get_model(&self, name: &str) -> Result<Arc<dyn Model>>;
// }
