use std::{io, path::PathBuf};

use rkyv::rancor;
use thiserror::Error;

use crate::util::errors::UtilReadError;

#[derive(Debug, Error)]
pub enum BPlusTreeError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Root path is not a directory: {0}")]
    RootPathNotDirectory(PathBuf),

    #[error("Error reading metadata file: {0}")]
    MetaDataFileIo(#[source] io::Error),

    #[error("Invalid metadata file magic")]
    MetaDataFileInvalidMagic,

    #[error("Metadata file version mismatch, got: {0}")]
    MetaDataFileVersionMismatch(u16),

    #[error("This program was compiled with page size: {0}, but the page size is: {1}")]
    MetaDataInvalidPageSize(u64, u64),

    #[error("Error reading from file: {0}")]
    IoReadError(#[from] UtilReadError),

    #[error("rkyv error: {0}")]
    RkyvError(#[from] rancor::Error),
}

#[derive(Debug, Error)]
pub enum PageError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Error reading from file: {0}")]
    ReadError(#[from] UtilReadError),

    #[error("Unknown page file version: {0} in page: {1}")]
    UnknownVersion(u16, u64),

    #[error("Invalid hash in page: {0}")]
    InvalidHash(u64),

    #[error("Invalid file magic in page: {0}")]
    InvalidMagic(u64),

    #[error("rkyv error: {0}")]
    RkyvError(#[from] rancor::Error),
}
