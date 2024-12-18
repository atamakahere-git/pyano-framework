pub enum TextEmbeddingModels {
    MiniLMV6,
    MiniLMV12,
    // Add other text embedding models here in the future
}

pub enum ImageEmbeddingModels {
    CLIP, // Example for image embedding model
    // Add other image embedding models here in the future
}

pub enum EmbeddingModels {
    Text(TextEmbeddingModels),
    Image(ImageEmbeddingModels),
}

impl EmbeddingModels {
    pub fn base_url(&self) -> &str {
        match self {
            EmbeddingModels::Text(model) =>
                match model {
                    TextEmbeddingModels::MiniLMV6 => {
                        "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/"
                    }
                    TextEmbeddingModels::MiniLMV12 => {
                        "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/"
                    }
                }
            EmbeddingModels::Image(model) =>
                match model {
                    ImageEmbeddingModels::CLIP => {
                        "https://huggingface.co/openai/clip-vit-base-patch32/resolve/main/"
                    }
                }
        }
    }

    pub fn model_path(&self) -> String {
        match self {
            EmbeddingModels::Text(model) => ".pyano/models/text".to_string(),

            EmbeddingModels::Image(model) => ".pyano/models/image".to_string(),
        }
    }

    /// Returns the list of files required for each embedding model
    pub fn required_files(&self) -> &'static [&'static str] {
        match self {
            EmbeddingModels::Text(model) =>
                match model {
                    TextEmbeddingModels::MiniLM =>
                        &[
                            "1_Pooling/config.json",
                            "config.json",
                            "config_sentence_transformers.json",
                            "data_config.json",
                            "modules.json",
                            "rust_model.ot",
                            "sentence_bert_config.json",
                            "special_tokens_map.json",
                            "tokenizer.json",
                            "tokenizer_config.json",
                            "vocab.txt",
                        ],
                }
            EmbeddingModels::Image(model) =>
                match model {
                    ImageEmbeddingModels::CLIP =>
                        &[
                            "config.json",
                            "merges.txt",
                            "pytorch_model.bin",
                            "special_tokens_map.json",
                            "tokenizer.json",
                            "vocab.json",
                        ],
                }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_model_base_url() {
        let text_model = EmbeddingModels::Text(TextEmbeddingModels::MiniLMV6);
        assert_eq!(
            text_model.base_url(),
            "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/"
        );
    }

    #[test]
    fn test_text_model_path() {
        let text_model = EmbeddingModels::Text(TextEmbeddingModels::MiniLMV6);
        assert_eq!(text_model.model_path(), ".pyano/models/embed_model_minilm");
    }

    #[test]
    fn test_text_model_required_files() {
        let text_model = EmbeddingModels::Text(TextEmbeddingModels::MiniLMV6);
        let expected_files = &[
            "1_Pooling/config.json",
            "config.json",
            "config_sentence_transformers.json",
            "data_config.json",
            "modules.json",
            "rust_model.ot",
            "sentence_bert_config.json",
            "special_tokens_map.json",
            "tokenizer.json",
            "tokenizer_config.json",
            "vocab.txt",
        ];
        assert_eq!(text_model.required_files(), expected_files);
    }

    #[test]
    fn test_image_model_base_url() {
        let image_model = EmbeddingModels::Image(ImageEmbeddingModels::CLIP);
        assert_eq!(
            image_model.base_url(),
            "https://huggingface.co/openai/clip-vit-base-patch32/resolve/main/"
        );
    }

    #[test]
    fn test_image_model_path() {
        let image_model = EmbeddingModels::Image(ImageEmbeddingModels::CLIP);
        assert_eq!(image_model.model_path(), ".pyano/models/embed_model_clip");
    }

    #[test]
    fn test_image_model_required_files() {
        let image_model = EmbeddingModels::Image(ImageEmbeddingModels::CLIP);
        let expected_files = &[
            "config.json",
            "merges.txt",
            "pytorch_model.bin",
            "special_tokens_map.json",
            "tokenizer.json",
            "vocab.json",
        ];
        assert_eq!(image_model.required_files(), expected_files);
    }
}
