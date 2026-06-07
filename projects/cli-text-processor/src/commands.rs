//! Commands for the text processor CLI.

use crate::error::ArgsError;
use std::fmt;

/// Represents a command for the text processor CLI.
#[derive(Debug)]
pub enum CommandType {
    Stats,
    Find(String),
    Top(usize),
    Filter(String),
    Longest,
    Unique,
    Replace(String, String),
}

pub struct Command {
    pub cmd: CommandType,
    pub case_sensitive: bool,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            cmd: CommandType::Stats,
            case_sensitive: false,
        }
    }
}

impl Command {
    pub fn new(cmd: &str, options: &[&str], flags: &[&str]) -> Result<Self, ArgsError> {
        let no_options = options.is_empty();
        let ignore_case = flags.contains(&"--ignore-case");

        match cmd {
            "stats" => Ok(Command::default()),
            "longest-line" => Ok(Self {
                cmd: CommandType::Longest,
                ..Command::default()
            }),
            "unique-lines" => Ok(Self {
                cmd: CommandType::Unique,
                ..Command::default()
            }),
            "find" | "top-words" | "filter-lines" | "replace" if no_options => {
                Err(ArgsError::MissingOption)
            }
            "find" if !no_options => Ok(Self {
                cmd: CommandType::Find(options[0].to_string()),
                case_sensitive: !ignore_case,
                ..Command::default()
            }),
            "top-words" if !no_options => {
                let count = options[0].parse::<usize>()?;
                Ok(Self {
                    cmd: CommandType::Top(count),
                    ..Command::default()
                })
            }
            "filter-lines" if !no_options => Ok(Self {
                cmd: CommandType::Filter(options[0].to_string()),
                case_sensitive: !ignore_case,
                ..Command::default()
            }),
            "replace" if !no_options && options.len() < 2 => Err(ArgsError::MissingOption),
            "replace" if !no_options && options.len() > 1 => {
                let pattern = options[0].to_string();
                let replacement = options[1].to_string();
                Ok(Self {
                    cmd: CommandType::Replace(pattern, replacement),
                    ..Command::default()
                })
            }
            _ => Err(ArgsError::Invalid),
        }
    }

    pub fn print_help(&self, cmd: &str) {
        match cmd {
            "stats" => println!("'stats': Print general statistics about the file."),
            "longest-line" => println!("'longest-line': Print the longest line in the file."),
            "unique-lines" => println!("'unique-lines': Print unique lines in the file."),
            "find" => println!("'find <pattern>': Find lines containing a specific pattern."),
            "top-words" => {
                println!("'top-words <count>': Print the top N most frequent words in the file.")
            }
            "filter-lines" => {
                println!("'filter-lines <pattern>': Filter lines containing a specific pattern.")
            }
            "replace" => println!(
                "'replace <pattern> <replacement>': Replace occurrences of a pattern with a replacement."
            ),
            x => println!("Unsupported command: {}", x),
        }
    }

    pub fn print_help_all(&self) {
        println!("Available commands:");
        for cmd in [
            "stats",
            "longest-line",
            "unique-lines",
            "find",
            "top-words",
            "filter-lines",
            "replace",
        ] {
            self.print_help(cmd);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    #[test]
    fn should_be_invalid_arg_empty() {
        let cmd = ProcessorCommand::new("", &[]).unwrap_err();
        assert_matches!(cmd, ArgsError::Invalid);
    }
}
