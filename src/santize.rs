use serde_json::Value;
use std::io::{self, Write}; // For flushing stdout

pub fn process_streamed_data(stream: &str, json_buffer: &mut String) -> Result<String, Box<dyn std::error::Error>> {
    let mut final_output = String::new();
    let segments: Vec<&str> = stream.split("data: ").collect();
     // Correctly print the segments with pretty-printing
    //  println!("Segments: {:#?}", segments);
    for segment in segments.iter().filter(|s| !s.is_empty()) {
        json_buffer.push_str(segment);

        // println!("            json_buffer: {:#?}", json_buffer);
        while let Some(pos) = json_buffer.rfind(|c|c == '}') {
            let temp_buffer = json_buffer.clone();
            let possible_json_str = &temp_buffer[..=pos];
            // println!("           possible_json_str: {:#?}", possible_json_str);
            match serde_json::from_str::<Value>(possible_json_str) {
                Ok(json) => {
                    if let Some(content) = json["choices"].get(0)
                        .and_then(|choice| choice["delta"]["content"].as_str()) {
                        final_output.push_str(content);
                        final_output.push(' '); // Add space between contents
                    }

                    // *json_buffer = temp_buffer[pos+1..].to_string();
                      json_buffer.clear();
                    //   println!("--------------------------------")
                },
                Err(err) => {
                    // println!("not done {}", err);
                    // If parsing fails, keep accumulating in the buffer for the next call
                    break;
                },
            }
        }
    }

    // println!("Final Output: {}", final_output.trim());
    // io::stdout().flush()?;

    Ok(final_output)
}
