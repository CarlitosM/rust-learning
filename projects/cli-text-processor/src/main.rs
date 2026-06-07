use std::env::{self, Args};

use crate::commands::ProcessorCommand;

mod commands;
mod error;
mod processor;

fn main() {
    let args = self::env::args().skip(1).collect::<Vec<String>>();
    let _ = handle_args(&args);
}

enum ArgType<'a> {
    Command(&'a str),
    File(&'a str),
    Param(&'a str),
    Flag(&'a str),
}

#[derive(Debug)]
struct Config {
    command: commands::ProcessorCommand,
    file_name: Option<String>,
    case_insensitive: bool,
}

impl Config {
    fn new<T: DoubleEndedIterator<Item = String>>(args: &T) -> Result<Self, error::ArgsError> {
        let mut args = args.skip(1);
        let file_or_cmd = args
            .next()
            .map(|arg| {
                if arg.ends_with(".txt") {
                    ArgType::File(&arg)
                } else {
                    ArgType::Command(&arg)
                }
            })
            .ok_or(error::ArgsError::NoArgs)?;
    }
}

fn handle_args(
    args: &Vec<String>,
) -> Result<(commands::ProcessorCommand, Option<String>), error::ArgsError> {
    if args.is_empty() {
        return Err(error::ArgsError::NoArgs);
    }

    let mut args = args.iter();
    let file_or_cmd = args.next().ok_or(error::ArgsError::NoArgs)?;
    let is_file = file_or_cmd.ends_with(".txt");

    let cmd_or_param = match args.next() {
        Some(val) => Ok(val.as_str()),
        None => {
            if is_file {
                Err(error::ArgsError::MissingCmd)
            } else {
                Ok(file_or_cmd.as_str())
            }
        }
    }?;

    let possibly_param = args.next();

    let arg_file = if is_file {
        Some(file_or_cmd.clone())
    } else {
        None
    };
    let arg_cmd = if is_file {
        cmd_or_param
    } else {
        file_or_cmd.as_str()
    };
    let param = if is_file {
        possibly_param.map(|p| p.as_str())
    } else {
        Some(cmd_or_param)
    };

    let cmd = ProcessorCommand::new(arg_cmd, param)?;

    Ok((cmd, arg_file))
}
