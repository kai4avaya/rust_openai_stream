
pub struct Interaction {
    pub command: String,
    pub response: String,
}

impl Interaction {
    pub fn new(command: String, response: String) -> Self {
        Self { command, response }
    }
}
