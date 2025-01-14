#[derive(Debug)]
pub struct ErrorAttributes {
    code: String,
    message: String,
}
impl ErrorAttributes {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message
        }
    }

    // Getter for code
    pub fn get_code(&self) -> &str {
        &self.code
    }

    // Getter for message
    pub fn get_message(&self) -> &str {
        &self.message
    }
}