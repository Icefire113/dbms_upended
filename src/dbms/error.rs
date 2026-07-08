use std::{ffi::OsString, io};

use thiserror::Error;

use crate::db::error::DBFormatError;

#[derive(Debug, Error)]
pub enum DBMSError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("Database root does not exist or has not been created")]
    DbmsRootDoesNotExist,

    #[error("Database name is not UTF-8: {:?}", _0)]
    DbNameIsNotUtf8(OsString),

    #[error("Error loading database")]
    DbLoadError(#[from] DBFormatError),
}
