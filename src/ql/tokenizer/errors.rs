use thiserror::Error;

/// Represents an error that occurs while tokenizing
#[derive(Debug, Error)]
pub enum SQLTokenizeError {
    #[error("Illegal token `{0}` at: {1}:{2}")]
    IllegalToken(String, usize, usize),

    #[error("Unknown token `{0}` at position: {1}:{2}")]
    UnknownToken(String, usize, usize),
}
