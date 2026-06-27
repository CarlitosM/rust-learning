//! Error types for CLI argument parsing.

use std::{error, fmt, io::Error as IoError, num::ParseIntError};

/// Name of a supported command associated with an argument parsing error.
#[derive(Debug)]
pub struct ValidCommand(pub String);

/// Parameter value that failed validation or parsing.
#[derive(Debug)]
pub struct InvalidParam(pub String);

/// Errors that can occur while parsing CLI arguments.
#[derive(Debug)]
pub enum ArgsError {
    /// No arguments were provided.
    NoArgs,
    /// An argument or command name is not supported.
    Invalid,
    /// A command argument was required but not provided.
    MissingCmd,
    /// A command option was required but not provided for the contained command.
    MissingOption(ValidCommand),
    /// A command option was provided for the contained command but could not be parsed.
    InvalidOption(ValidCommand, InvalidParam),
}

impl error::Error for ArgsError {}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgsError::NoArgs => write!(f, "one argument required"),
            ArgsError::Invalid => write!(f, "invalid argument"),
            ArgsError::MissingOption(cmd) => write!(f, "Option required for command: {}", cmd.0),
            ArgsError::InvalidOption(cmd, param) => write!(
                f,
                "invalid Option for command: {} param: {}",
                cmd.0, param.0
            ),
            ArgsError::MissingCmd => write!(f, "command required"),
        }
    }
}

impl From<ParseIntError> for ArgsError {
    fn from(err: ParseIntError) -> Self {
        ArgsError::InvalidOption(
            ValidCommand("top-words".to_string()),
            InvalidParam(err.to_string()),
        )
    }
}

#[derive(Debug)]
pub enum ProcessError {
    FileError(IoError),
}

impl error::Error for ProcessError {}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::FileError(err) => write!(f, "file error: {err}"),
        }
    }
}

impl From<IoError> for ProcessError {
    fn from(err: IoError) -> Self {
        ProcessError::FileError(err)
    }
}
