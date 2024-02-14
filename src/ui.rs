use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::stdout;
use crate::Result;

use tokio::sync::mpsc::{Receiver, Sender}; // Import from tokio
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::task;
enum UIEvent {
    Quit,
    // Define other UI events as needed
}

/// Initializes the terminal by enabling raw mode and entering an alternate screen. 
pub fn initialize_terminal() -> Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(())
}

/// Cleans up the terminal by disabling raw mode and leaving the alternate screen.
pub fn cleanup_terminal() -> Result<()> {
    let mut stdout = stdout();
    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

/// This function reads user input from stdin in a separate thread 
/// and sends it to the main thread through a channel (Sender<String>). It continuously loops, reading input until an error occurs or the user quits.
pub async fn read_user_input(tx: Sender<String>) -> io::Result<()> {
    let stdin = io::stdin(); // Use Tokio's stdin for async reading
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let line = line.trim().to_string();
        if line == "/exit" {
            break;
        }

        // Using spawn_blocking to accommodate the synchronous send in an async context
        let tx_clone = tx.clone();
        task::spawn_blocking(move || {
            if tx_clone.blocking_send(line).is_err() {
                println!("Failed to send user input to main thread");
                // Handle error, potentially breaking out or logging
            }
        }).await?;
    }

    Ok(())
}


/// This function receives user input from the main thread through a channel (Receiver<String>) and handles UI events accordingly. 
pub async fn handle_ui_events(mut rx: Receiver<String>) {
    while let Some(input) = rx.recv().await {
        match input.as_str() {
            "/quit" => {
                println!("Quitting...");
                break;
            },
            _ => {
                println!("Received input: {}", input);
                // Handle other inputs as needed
            },
        }
    }
}
