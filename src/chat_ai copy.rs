// use reqwest;
// use reqwest::{Client, Error};
// use anyhow::anyhow; // Correctly import the anyhow function'
// use serde_json::json;
use anyhow::Result; // Make sure to include anyhow in your Cargo.toml
use serde::{Serialize, Deserialize};
use tokio::macros::support::Future;
use serde_json;
use serde_json::Value;
// use serde_json::Value;
use anyhow::anyhow;
use futures::StreamExt;
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagesContainer {
    messages: Vec<Message>,
}

pub async fn query_chatgpt<F, Fut>(query: &str, mut callback: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(String) -> Fut,
    Fut: Future<Output = ()> + Send,
{
    let messages = MessagesContainer {
        messages: vec![
            Message {
                role: "user".to_owned(),
                content: query.to_owned(), // Convert &str to String
            },
        ],
    };

    
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");
    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    let api_key = api_key; 

let response = client
.post(url)
.header("Authorization", format!("Bearer {}", api_key))
.json(&serde_json::json!({
    "model": "gpt-3.5-turbo",
    "stream": true,
    "messages": [{
        "role": "user",
        "content": query
    }]
}))
.send()
.await?;

if !response.status().is_success() {
    return Err(anyhow!("API call failed with status: {}", response.status()).into());
}

// Initialize an empty string to aggregate contents
// let mut aggregated_contents = String::new();

let mut stream = response.bytes_stream();

while let Some(chunk) = stream.next().await {
    let chunk = chunk.map_err(|e| anyhow!("Stream error: {}", e))?;
    let text = String::from_utf8(chunk.to_vec()).map_err(|e| anyhow!("Invalid UTF-8 sequence: {}", e))?;

    // Check if the chunk matches "[DONE]"
    if text.trim() == "[DONE]" {
        callback("DONE".to_string()).await; // Await the callback invocation
        continue;
    }

    if let Ok(json) = serde_json::from_str::<Value>(&text) {
        if let Some(contents) = json["choices"][0]["delta"]["content"].as_str() {
            callback(contents.to_string()).await; // Await the callback invocation
        }
    }
}

Ok(())
}