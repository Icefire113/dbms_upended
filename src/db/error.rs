use std::io;

use thiserror::Error;

use crate::util::errors::UtilReadError;

#[derive(Debug, Error)]
pub enum DBFormatError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("Database format file is corrupted")]
    DbFormatFileCorrupted,

    #[error("Checksum for database format file does not match")]
    DbFormatFileInvalidChecksum,

    #[error("Unknown database format file version: {0}")]
    DbFormatFileUnknownVersion(u32),

    #[error("Invalid database format file magic")]
    DbFormatInvalidHeaderMagic,

    #[error("Mismatched database name in format file")]
    DbFormatMismatchedName,

    #[error("Mismatched table name in format file")]
    DbFormatMismatchedTableName,

    #[error("Database format file not found")]
    LoadingDbFormatFile(#[source] io::Error),

    #[error("Error reading")]
    UtilReadError(#[from] UtilReadError),

    #[error("Decoding error")]
    DecodeError(#[from] bitcode::Error),
}
