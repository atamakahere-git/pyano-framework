use log::info;
use bytes::Bytes;

// use serde_json::json;
use serde::Deserialize;

use futures::{ Stream, StreamExt }; // Ensure StreamExt is imported
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct LLMGenerattionTimings {
    predicted_ms: f64,
    predicted_n: f64,
    predicted_per_second: f64,
    predicted_per_token_ms: f64,
    prompt_ms: f64,
    prompt_n: f64,
    prompt_per_second: f64,
    prompt_per_token_ms: f64,
}

pub fn llamacpp_process_stream<'a>(
    stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Unpin + 'a
) -> impl Stream<Item = Result<Bytes, reqwest::Error>> + 'a {
    let acc = String::new(); // Initialize accumulator

    futures::stream::unfold((stream, acc), |(mut stream, acc)| async move {
        if let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    if let Ok(chunk_str) = std::str::from_utf8(&chunk) {
                        let content_to_stream = process_chunk(chunk_str).await;

                        if !content_to_stream.is_empty() {
                            return Some((Ok(Bytes::from(content_to_stream)), (stream, acc)));
                        }
                    } else {
                        eprintln!("Failed to parse chunk as UTF-8");
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving chunk: {}", e);
                    return Some((Err(e), (stream, acc)));
                }
            }
        } else {
            // End of stream
            return None;
        }

        Some((Ok(Bytes::new()), (stream, acc)))
    })
}

async fn process_chunk(chunk_str: &str) -> String {
    let mut content_to_stream = String::new();

    for line in chunk_str.lines() {
        if line.starts_with("data: ") {
            if let Ok(json_data) = serde_json::from_str::<Value>(&line[6..]) {
                if let Some(content) = json_data.get("content").and_then(|c| c.as_str()) {
                    content_to_stream.push_str(content); // Stream content
                }
                if let Some(timings) = json_data.get("timings") {
                    if
                        let Ok(timing_struct) = serde_json::from_value::<LLMGenerattionTimings>(
                            timings.clone()
                        )
                    {
                        let tokens_per_second = calculate_tokens_per_second(
                            timing_struct.predicted_n,
                            timing_struct.predicted_ms
                        );
                        info!("Tokens generated per second: {:.2}", tokens_per_second);
                    }
                }
            }
        }
    }
    content_to_stream
}

fn calculate_tokens_per_second(predicted_n: f64, predicted_ms: f64) -> f64 {
    let predicted_seconds = predicted_ms / 1000.0;
    predicted_n / predicted_seconds
}
