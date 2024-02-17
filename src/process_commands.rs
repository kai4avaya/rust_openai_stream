use crate::chat_ai::query_chatgpt;
use crate::clipboard_utils::copy_interactions_to_clipboard;
use crate::santize::process_streamed_data;
use crate::Interaction;
use anyhow::Result;
use futures::StreamExt; // For processing the stream
use std::io::{self, Write}; // Import Write trait for flush() method
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex; // Adjust the path according to where Interaction is defined
struct SharedState {
    response_buffer: String,          // For the latest response
    interaction_history: Vec<String>, // To store history of all interactions
}

pub async fn process_commands(
    mut async_rx: Receiver<char>,
    interactions: Arc<Mutex<Vec<Interaction>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut command_buffer = String::new();
    // let shared_state = Arc::new(Mutex::new(SharedState {
    //     response_buffer: String::new(),
    //     interaction_history: Vec::new(),
    // }));

    // let mut chat_response_buffer = String::new();

    while let Some(char_received) = async_rx.recv().await {
        match char_received {
            '\x1F' => {
                // Explicit Enter press signal
                println!("");
                if command_buffer.starts_with('/') {
                    let command = command_buffer.clone();
                    // Clear the command_buffer for the next command
                    command_buffer.clear();
                    let interactions_clone = Arc::clone(&interactions);
                    match command.as_str() {
                        // Inside the match block for command processing
                        "/copy" => {
                            let mut num_to_copy = 1;
                        
                            // Extract number from command if present
                            // Check if there's a space after "/copy", indicating "/copy #" format
                            // or if the command directly continues with a number, indicating "/copy#" format
                            let command_trimmed = command.trim_start_matches("/copy").trim_start();
                            if !command_trimmed.is_empty() {
                                // Attempt to parse the remaining string as a number
                                if let Ok(num) = command_trimmed.parse::<i32>() {
                                    num_to_copy = num.max(1); // Ensure num_to_copy is at least 1
                                } else {
                                    // Attempt to handle "/copy #" format by splitting the command by whitespace
                                    // and parsing the second part as a number
                                    let parts: Vec<&str> = command.split_whitespace().collect();
                                    if parts.len() > 1 && parts[0] == "/copy" {
                                        if let Ok(num) = parts[1].parse::<i32>() {
                                            num_to_copy = num.max(1); // Ensure num_to_copy is at least 1
                                        }
                                    }
                                }
                            }
                        
                            // Lock the interactions mutex and access the data
                            let interactions_guard = interactions.lock().await;
                            let interactions_slice = &*interactions_guard; // Dereference the guard, then borrow it to get a slice
                        
                            // Now pass the slice to the function
                            if let Err(e) = copy_interactions_to_clipboard(interactions_slice, num_to_copy) {
                                eprintln!("Error copying interactions to clipboard: {}", e);
                            }
                            println!("Content copied to clipboard.");
                        },
                        
                        

                        "/exit" => {
                            println!("Exiting application.");
                            std::process::exit(0);
                        }
                        _ => {
                            // For other commands starting with '/'
                            // let shared_state_clone = Arc::clone(&shared_state);

                            // let interactions_clone = Arc::clone(&interactions);

                            tokio::spawn(async move {
                                let mut json_buffer = String::new(); // Add this line
                                let mut stream = query_chatgpt(&command);
                                // let mut json_accumulation_started = false;
                                let mut temp_responses = Vec::new();
                                // Temporary storage for interactions to minimize lock duration
                                let mut temp_interactions: Vec<String> = Vec::new();

                                while let Some(result) = stream.next().await {
                                    match result {
                                        Ok(chunk) => {
                                            // println!("------------");

                                            //  println!("{:?}", chunk);
                                            // println!("------------");

                                            if chunk == "[DONE]" {
                                                println!("Stream complete.");
                                             
                                                break;
                                            } else if chunk.contains("data: [DONE]") {
                                                   // Combine the responses into a single Interaction and add it to interactions
                                                   let full_response: String = temp_responses.join("\n");
     
                                                   println!("Full response: {}", full_response);
                                                   let mut interactions =
                                                       interactions_clone.lock().await;
                                                   interactions.push(Interaction::new(full_response));
   
                                                println!();
                                                break;
                                            } else {
                                                match process_streamed_data(
                                                    &chunk,
                                                    &mut json_buffer,
                                                ) {
                                                    Ok(content) => {
                                                        // println!("Extracted Content: {}", content);
                                                        // io::stdout().flush().unwrap();
                                                        print!("{}", content); // No newline character here
                                                        io::stdout().flush().unwrap(); // Flush to ensure it appears immediately
                                                                                       // Store in temporary vector
                                                                                       //   temp_interactions.push(Interaction::new(content));
                                                                                       // Update the interactions with the content received
                                                        temp_responses.push(content);
                                                    }
                                                    Err(e) => eprintln!(
                                                        "Error processing stream data: {}",
                                                        e
                                                    ),
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Error processing response: {}", e);
                                        }
                                    }
                                }
                            });
                        }
                    }
                } else {
                    // If the buffer does not start with '/', just echo it back
                }
                // command_buffer.clear();
            }
            '\n' => { // Handle newline characters if necessary
                 // This might be part of pasted input, so you could decide how to handle it.
                 // For example, you might want to ignore it or handle it differently.
            }
            '\x08' => {
                // Backspace control character
                // Remove the last character from the command buffer
                command_buffer.pop();
            }
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
