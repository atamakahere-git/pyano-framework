use crate::llm::options::LLMHTTPCallOptions;
use std::error::Error as StdError; // Importing the correct trait
use std::pin::Pin;
use bytes::Bytes;
use futures::{ Stream, StreamExt }; // Ensure StreamExt is imported

pub struct LLM {
    client: reqwest::Client,
    options: LLMHTTPCallOptions,
}

impl LLM {
    pub fn builder() -> LLMBuilder {
        LLMBuilder::default()
    }

    async fn prepare_request(
        &self,
        prompt_with_context: &str,
        system_prompt: &str,
        stream: bool
    ) -> Result<reqwest::Response, Box<dyn StdError + Send + Sync + 'static>> {
        let server_url = self.options.server_url.as_ref().expect("Server URL is missing");

        let prompt_template = self.options.prompt_template
            .as_ref()
            .expect("Prompt template is missing");

        let full_prompt = prompt_template
            .replace("{system_prompt}", system_prompt)
            .replace("{user_prompt}", prompt_with_context);

        let mut json_payload = serde_json::Map::new();
        json_payload.insert("prompt".to_string(), serde_json::Value::String(full_prompt));
        json_payload.insert("stream".to_string(), serde_json::Value::Bool(stream));
        json_payload.insert("cache_prompt".to_string(), serde_json::Value::Bool(true));

        if let Some(temperature) = self.options.temperature {
            json_payload.insert(
                "temperature".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(temperature as f64).unwrap())
            );
        }
        if let Some(top_k) = self.options.top_k {
            json_payload.insert(
                "top_k".to_string(),
                serde_json::Value::Number(serde_json::Number::from(top_k as i64))
            );
        }
        if let Some(top_p) = self.options.top_p {
            json_payload.insert(
                "top_p".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(top_p as f64).unwrap())
            );
        }
        if let Some(seed) = self.options.seed {
            json_payload.insert(
                "seed".to_string(),
                serde_json::Value::Number(serde_json::Number::from(seed as i64))
            );
        }
        if let Some(min_length) = self.options.min_length {
            json_payload.insert(
                "min_length".to_string(),
                serde_json::Value::Number(serde_json::Number::from(min_length as i64))
            );
        }
        if let Some(max_length) = self.options.max_length {
            json_payload.insert(
                "max_length".to_string(),
                serde_json::Value::Number(serde_json::Number::from(max_length as i64))
            );
        }
        if let Some(repetition_penalty) = self.options.repetition_penalty {
            json_payload.insert(
                "repetition_penalty".to_string(),
                serde_json::Value::Number(
                    serde_json::Number::from_f64(repetition_penalty as f64).unwrap()
                )
            );
        }

        let resp = self.client
            .post(&format!("{}/completion", server_url))
            .json(&serde_json::Value::Object(json_payload))
            .send().await?
            .error_for_status()?;

        Ok(resp)
    }

    pub async fn response_stream(
        &self,
        prompt_with_context: &str,
        system_prompt: &str
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
        Box<dyn StdError + Send + Sync + 'static>
    > {
        let resp = self.prepare_request(prompt_with_context, system_prompt, true).await?;

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            let mut stream = resp.bytes_stream(); // Use `bytes_stream` as `chunk` method is not available for all reqwest versions
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        if tx.send(Ok(bytes)).await.is_err() {
                            eprintln!("Receiver dropped");
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        break;
                    }
                }
            }
        });

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    pub async fn response(
        &self,
        prompt_with_context: &str,
        system_prompt: &str
    ) -> Result<serde_json::Value, Box<dyn StdError + Send + Sync + 'static>> {
        let resp = self.prepare_request(prompt_with_context, system_prompt, false).await?;
        let response_json = resp.json::<serde_json::Value>().await?;
        Ok(response_json)
    }
}

pub struct LLMBuilder {
    options: LLMHTTPCallOptions,
}

impl Default for LLMBuilder {
    fn default() -> Self {
        LLMBuilder {
            options: LLMHTTPCallOptions::new(),
        }
    }
}

impl LLMBuilder {
    pub fn with_options(mut self, options: LLMHTTPCallOptions) -> Self {
        self.options = options;
        self
    }

    pub fn build(self) -> LLM {
        LLM {
            client: reqwest::Client::new(),
            options: self.options.build(),
        }
    }
}
