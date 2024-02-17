

use clipboard::{ClipboardProvider, ClipboardContext};
use std::error::Error;
use crate::Interaction; // Adjust the path according to where Interaction is defined

pub fn copy_interactions_to_clipboard(interactions: &[Interaction], mut num_to_copy: i32) -> Result<(), Box<dyn Error>> {
    if interactions.is_empty() {
        return copy_to_clipboard("");
    }

    // Ensure num_to_copy is within valid range, defaulting to 1 if it's 0 or negative
    if num_to_copy <= 0 {
        num_to_copy = 1;
    }

    let valid_num_to_copy = num_to_copy.min(interactions.len() as i32) as usize;

    // Determine the start index for copying, ensuring it's not negative
    let start_index = interactions.len().saturating_sub(valid_num_to_copy);

    // Take the last num_to_copy interactions, starting from start_index
    let text_to_copy = interactions[start_index..]
                        .iter()
                        .map(|interaction| interaction.response.as_str())
                        .collect::<Vec<&str>>()
                        .join("\n");

    println!("text_to_copy: {}", text_to_copy);
    copy_to_clipboard(&text_to_copy)
}


pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(text.to_owned())?;
    
    Ok(())
}
