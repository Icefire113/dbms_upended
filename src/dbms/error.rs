use std::{ffi::OsString, io};

use thiserror::Error;

use crate::util::errors::UtilReadError;

#[derive(Debug, Error)]
pub enum DBMSError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("Database root does not exist or has not been created")]
    DbmsRootDoesNotExist,

    #[error("Database name is not UTF-8: {:?}", _0)]
    DbNameIsNotUtf8(OsString),

    #[error("Error loading database: {0}")]
    DbLoadError(#[from] DBFormatLoadError),

    #[error("Error saving database: {0}")]
    DbSaveError(#[from] DBFormatSaveError),
}

#[derive(Debug, Error)]
pub enum DBFormatLoadError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("Database format file is corrupted")]
    DbFormatFileCorrupted,

    #[error("Checksum for database format file does not match")]
    DbFormatFileInvalidChecksum,

    #[error(
        "Unknown database format file version: {0}, it may have been created with a newer version"
    )]
    DbFormatFileUnknownVersion(u32),

    #[error("Invalid database format file magic")]
    DbFormatInvalidHeaderMagic,

    #[error("Database format file not found")]
    LoadingDbFormatFile(#[source] io::Error),

    #[error("Error reading")]
    UtilReadError(#[from] UtilReadError),

    #[error("Decoding error")]
    DecodeError(#[from] bitcode::Error),
}

#[derive(Debug, Error)]
pub enum DBFormatSaveError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
}
