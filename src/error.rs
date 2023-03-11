use thiserror::Error;

use crate::{buffer::BufferError, ArgError, ParserError};

pub type ReplResult<T> = std::result::Result<T, ReplError>;

#[derive(Debug, Error)]
pub enum ReplError {
    #[error("Unrecoverable readline error: {0}")]
    EditorError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parameter error: {0}")]
    ArgError(#[from] ArgError),

    #[error("No such command: {0}")]
    NoSuchCommandError(String),

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),

    #[error("Parser error: {0}")]
    ParserError(#[from] ParserError),
}
