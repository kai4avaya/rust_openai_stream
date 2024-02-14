use tokio::sync::mpsc::Receiver;
use crate::chat_ai::query_chatgpt;
use crate::clipboard_utils::copy_to_clipboard;
use crate::santize::process_streamed_data;
use anyhow::Result;
use std::io::{self, Write}; // Import Write trait for flush() method
use tokio::sync::Mutex;
use std::sync::Arc;
use futures::StreamExt; // For processing the stream
use crate::Interaction; // Adjust the path according to where Interaction is defined
use serde_json::Value;
struct SharedState {
    response_buffer: String,
    // Include other shared data as needed
}

pub async fn process_commands(
    mut async_rx: Receiver<char>,
    interactions: Arc<Mutex<Vec<Interaction>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut command_buffer = String::new();
    let shared_state = Arc::new(Mutex::new(SharedState {
        response_buffer: String::new(),
    }));

    while let Some(char_received) = async_rx.recv().await {
        match char_received {
            '\x1F' => { // Explicit Enter press signal
                println!("");
                if command_buffer.starts_with('/') {
                    let command = command_buffer.clone();
                    // Clear the command_buffer for the next command
                    command_buffer.clear();

                    match command.as_str() {
                        "/copy" => {
                            let _ = copy_to_clipboard(&command); // Adjust with actual terminal content variable
                            println!("Content copied to clipboard."); // Placeholder for actual copy action
                        },
                        "/exit" => {
                            println!("Exiting application.");
                            std::process::exit(0);
                        },
                        _ => { // For other commands starting with '/'
                            // let shared_state_clone = Arc::clone(&shared_state);
                            // let interactions_clone = Arc::clone(&interactions);
                            tokio::spawn(async move {
                                let mut json_buffer = String::new(); // Add this line
                                let mut stream = query_chatgpt(&command);
                                // let mut json_accumulation_started = false;

                                while let Some(result) = stream.next().await {
                                    match result {
                                        Ok(chunk) => {
                                            // println!("------------");

                                            //  println!("{:?}", chunk);
                                            // println!("------------");


                                            if chunk == "[DONE]" {
                                                println!("Stream complete.");
                                                break;
                                            }
                                            else if chunk.contains("data: [DONE]") {
                                                println!("dd");
                                                break;
                                            }
                                            else {
                                                match process_streamed_data(&chunk,  &mut json_buffer) {
                                                    Ok(content) => {
                                                        // println!("Extracted Content: {}", content);
                                                        // io::stdout().flush().unwrap();
                                                        print!("{}", content); // No newline character here
                                                        io::stdout().flush().unwrap(); // Flush to ensure it appears immediately
                                   
                                                    },
                                                    Err(e) => eprintln!("Error processing stream data: {}", e),
                                                }
                                           
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!("Error processing response: {}", e);
                                        },
                                    }
                                }
                            });
                        

                        }
                    }
                } else {
                    // If the buffer does not start with '/', just echo it back

                }
                // command_buffer.clear();
            },
            '\n' => { // Handle newline characters if necessary
                // This might be part of pasted input, so you could decide how to handle it.
                // For example, you might want to ignore it or handle it differently.
            },
            '\x08' => { // Backspace control character
                // Remove the last character from the command buffer
                command_buffer.pop();
            },
            _ => command_buffer.push(char_received), // Accumulate other characters
        }
    }

    Ok(())
}


fn response_is_complete(text: &str) -> bool {
    // Implement logic to determine if the response is complete
    // This could check for certain delimiters, JSON structures, etc.
    text.ends_with("DONE") // Example condition
}