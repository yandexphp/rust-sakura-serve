use std::fmt;

#[derive(Debug, Clone)]
pub struct CustomError {
    pub message: String,
    pub error_code: String,
}

impl CustomError {
    pub fn new(message: &str, error_code: &str) -> Self {
        CustomError {
            message: message.to_string(),
            error_code: error_code.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_code, self.message)
    }
}

impl std::error::Error for CustomError {}
