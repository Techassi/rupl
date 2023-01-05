use rustyline::error::ReadlineError;
use thiserror::Error;

use crate::ParameterError;

pub type ReplResult<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot add subcommands to a command with parameters")]
    IllegalSubcommandError,

    #[error("Cannot add parameters to a command with subcommands")]
    IllegalParameterError,

    #[error("Unrecoverable readline error: {0}")]
    EditorError(#[from] ReadlineError),

    #[error("Parameter error: {0}")]
    ParameterError(#[from] ParameterError),

    #[error("No such command: {0}")]
    NoSuchCommandError(String),
}
