use clipboard::{ClipboardProvider, ClipboardContext};
use std::error::Error;

pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn Error>> {
    // Attempt to create a new clipboard provider
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    
    // Try to set the clipboard contents to the provided text
    ctx.set_contents(text.to_owned())?;
    
    Ok(())
}
