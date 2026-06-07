use std::{error, fmt, num::ParseIntError};

#[derive(Debug)]
pub enum ArgsError {
    NoArgs,
    Invalid,
    MissingCmd,
    MissingOption,
    InvalidOption,
}

impl error::Error for ArgsError {}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgsError::NoArgs => write!(f, "one argument required"),
            ArgsError::Invalid => write!(f, "invalid argument"),
            ArgsError::MissingOption => write!(f, "Option required"),
            ArgsError::InvalidOption => write!(f, "invalid Option"),
            ArgsError::MissingCmd => write!(f, "command required"),
        }
    }
}

impl From<ParseIntError> for ArgsError {
    fn from(_: ParseIntError) -> Self {
        ArgsError::InvalidOption
    }
}
