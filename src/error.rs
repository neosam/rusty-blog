//! Contains types required to handle errors

/// Custom error if parsing failed
///
/// It contains a String to describe the error in more detail.
#[derive(Debug)]
pub struct ParseError(pub String);
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}
impl std::error::Error for ParseError {}
impl ParseError {
    pub fn new(message: impl ToString) -> ParseError {
        ParseError(message.to_string())
    }
}

/// Default Result type for the error handling.
pub type BlogResult<T> = Result<T, Box<dyn std::error::Error>>;
