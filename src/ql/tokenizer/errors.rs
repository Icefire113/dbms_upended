use thiserror::Error;


/// Represents an error that occurs while tokenizing
#[derive(Debug, Error)]
pub enum SQLTokenizeError {
    #[error("Illegal token `{0}` at position: {1}")]
    IllegalToken(String, u64),

    #[error("Unknown token `{0}` at position: {1}")]
    UnknownToken(String, u64),
}
