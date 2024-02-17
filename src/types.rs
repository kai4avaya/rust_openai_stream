#[derive(Debug)]
pub struct Interaction {
    // pub command: String,
    pub response: String,
}

impl Interaction {
    pub fn new(response: String) -> Self {
        Self { response }
    }
}
