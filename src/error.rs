use rustyline::error::ReadlineError;
use thiserror::Error;

use crate::ArgError;

pub type ReplResult<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unrecoverable readline error: {0}")]
    EditorError(#[from] ReadlineError),

    #[error("Parameter error: {0}")]
    ArgError(#[from] ArgError),

    #[error("No such command: {0}")]
    NoSuchCommandError(String),
}
