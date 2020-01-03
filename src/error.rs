#[derive(Debug)]
pub struct ParseError(pub String);
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}
impl std::error::Error for ParseError {}

pub type BlogResult<T> = Result<T, Box<dyn std::error::Error>>;