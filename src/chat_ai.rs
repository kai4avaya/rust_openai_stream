use anyhow::{Result, anyhow};
use futures::{stream, Stream, StreamExt};
use reqwest::Client;
use serde::{Serialize, Deserialize};
// use serde_json::Value;
use std::pin::Pin;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagesContainer {
    messages: Vec<Message>,
}

// Adjusted return type to match the expected type
pub fn query_chatgpt(query: &str) -> Pin<Box<dyn Stream<Item = Result<String>> + Send>> {
    let messages = MessagesContainer {
        messages: vec![Message { role: "user".to_owned(), content: query.to_owned() }],
    };

    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => return Box::pin(stream::empty()), // Early return on error, adjusted to match the return type
    };

    let response_future = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "stream": true,
            "messages": messages.messages
        }))
        .send();

        let stream = Box::pin(async_stream::stream! {
            let response = match response_future.await {
                Ok(response) => response,
                Err(e) => {
                    yield Err(anyhow!(e));
                    return;
                }
            };
    
            if !response.status().is_success() {
                yield Err(anyhow!("API call failed with status: {}", response.status()));
                return;
            }
    
            let mut byte_stream = response.bytes_stream();
    
            while let Some(chunk) = byte_stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        let text = match String::from_utf8(bytes.to_vec()) {
                            // Ok(text) => text,
                            Ok(text) => {
                                // Debug print statement here
                                // println!("Debug: {}", text); // This line prints each chunk as it is received
                                text // This is the text being returned from the inner match arm
                            },
                            Err(e) => {
                                yield Err(anyhow!("Invalid UTF-8 sequence: {}", e));
                                continue;
                            }
                        };
                        yield Ok(text);
                    },
                    Err(e) => yield Err(anyhow!("Stream error: {}", e)),
                }
            }
        });
        
        stream
    }