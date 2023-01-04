use rustyline::error::ReadlineError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot add subcommands to a command with parameters")]
    IllegalSubcommandError,

    #[error("Cannot add parameters to a command with subcommands")]
    IllegalParameterError,

    #[error("Unrecoverable readline error: {0}")]
    EditorError(#[from] ReadlineError),
}
