//! Crate-wide error type.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BinliftError {
    #[error("unrecognized magic bytes: {0:x?}")]
    UnknownFormat(Vec<u8>),

    #[error("unexpected end of data at offset {0}")]
    Truncated(usize),

    #[error("unsupported feature: {0}")]
    Unsupported(String),
}

pub type Result<T> = std::result::Result<T, BinliftError>;