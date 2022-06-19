#[derive(Debug)]
pub struct ParsingError {
    pub line: u64,
    pub message: String,
}

impl ParsingError {
    pub fn new(line: u64, message: String) -> Self {
        ParsingError { line, message }
    }
}
