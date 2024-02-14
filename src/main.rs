use anyhow::Result;
use crossterm::{
    event::{ Event, KeyCode, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},

};
use std::time::{Instant, Duration};
use std::io::{self, Write};
use std::thread;
use tokio::sync::mpsc as async_mpsc;

use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

mod chat_ai;
mod clipboard_utils;
mod process_commands; // Declare the new module
mod santize;


use process_commands::process_commands; // Import the function
pub mod types;
use crate::types::Interaction;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let interactions = Arc::new(AsyncMutex::new(Vec::<Interaction>::new()));


    // Threshold for detecting rapid input, e.g., paste operation
    const INPUT_THRESHOLD: Duration = Duration::from_millis(10);
    let mut last_input_time = Instant::now();
    let mut is_rapid_input = false;

    // Assuming async_tx is a tokio::sync::mpsc::Sender<char>
    let (async_tx,  async_rx) = async_mpsc::channel::<char>(32);
    
    thread::spawn(move || {
        let mut stdout = io::stdout();
        // No need for input_buffer in this scenario
    
        loop {
            if let Event::Key(event) = read().unwrap() {
                let now = Instant::now();
                is_rapid_input = now.duration_since(last_input_time) < INPUT_THRESHOLD;
                last_input_time = now;
        
                match event.code {
                    KeyCode::Enter => {

                        if !is_rapid_input {
                            if async_tx.blocking_send('\x1F').is_err() {
                                println!("Error sending Enter pressed signal to async processing");
                            }
                            
                        }
                    else {
                               // Send a special character or command to indicate Enter was pressed
                               if async_tx.blocking_send('\n').is_err() {
                                println!("Error sending Enter to async processing");
                            }
                    }
                    },
                    KeyCode::Char(c) => {
                        if async_tx.blocking_send(c).is_err() {
                            println!("Error sending character to async processing");
                        }
                        print!("{}", c);
                        stdout.flush().unwrap();
                    },
                    KeyCode::Backspace => {
                        // Send backspace signal
                        if async_tx.blocking_send('\x08').is_err() {
                            println!("Error sending Backspace to async processing");
                        }
                    
                        // Move the cursor back by one character and clear that character
                        execute!(
                            stdout,
                            crossterm::cursor::MoveLeft(1),
                            crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine)
                        ).unwrap();
                    
                        stdout.flush().unwrap();
                    },
                    KeyCode::Esc => {
                        break;
                    },
                    _ => {}
                }
            }
        }
    });
    
    // Adjust process_commands signature to accept Receiver<char>
    if let Err(e) = process_commands(async_rx, Arc::clone(&interactions)).await {
        eprintln!("Error processing commands: {}", e);
    }
    

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
