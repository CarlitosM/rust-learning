//! Commands for the text processor CLI.

use crate::error::{ArgsError, ValidCommand};
use std::{fmt, io};

/// Represents a command for the text processor CLI.
#[derive(Debug)]
pub enum CommandType {
    /// Prints general statistics for the input text.
    Stats,
    /// Finds lines containing the given pattern.
    Find(String),
    /// Prints the top N most frequent words.
    Top(usize),
    /// Filters lines containing the given pattern.
    Filter(String),
    /// Prints the longest line in the input text.
    Longest,
    /// Prints unique lines from the input text.
    Unique,
    /// Replaces occurrences of the first string with the second string.
    Replace(String, String),
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandType::Stats => write!(f, "stats"),
            CommandType::Find(s) => write!(f, "find {}", s),
            CommandType::Top(n) => write!(f, "top-{}", n),
            CommandType::Filter(s) => write!(f, "filter {}", s),
            CommandType::Longest => write!(f, "longest-line"),
            CommandType::Unique => write!(f, "unique-lines"),
            CommandType::Replace(old, new) => write!(f, "replace {} {}", old, new),
        }
    }
}

/// Parsed CLI command configuration.
#[derive(Debug)]
pub struct Command {
    /// The operation to run.
    pub cmd: CommandType,
    /// Whether pattern matching should preserve letter case.
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
    /// Builds a command from the command name, positional options, and flags.
    ///
    /// Commands that require options return [`ArgsError::MissingOption`] with
    /// the command name when none are provided. `top-words` returns
    /// [`ArgsError::InvalidOption`] with the command name and parse failure when
    /// its count cannot be parsed as an unsigned integer, and unknown command
    /// names return [`ArgsError::Invalid`].
    pub fn new(cmd: &str, options: &Vec<&str>, flags: &Vec<&str>) -> Result<Self, ArgsError> {
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
                Err(ArgsError::MissingOption(ValidCommand(cmd.to_string())))
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
            "replace" if !no_options && options.len() < 2 => Err(ArgsError::MissingOption(
                ValidCommand("replace".to_string()),
            )),
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
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let case_sensitive = if self.case_sensitive {
            "(case sensitive)"
        } else {
            "(case insensitive)"
        };
        write!(f, "{} {}", self.cmd, case_sensitive)
    }
}

/// Prints command-specific help text for a command name.
#[allow(dead_code)]
pub fn print_help(cmd: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write_help(cmd, &mut handle).expect("failed to write command help");
}

/// Writes command-specific help text for a command name.
pub fn write_help<W: io::Write>(cmd: &str, writer: &mut W) -> io::Result<()> {
    match cmd {
        "stats" => writeln!(writer, "'stats': Print general statistics about the file."),
        "longest-line" => writeln!(
            writer,
            "'longest-line': Print the longest line in the file."
        ),
        "unique-lines" => writeln!(writer, "'unique-lines': Print unique lines in the file."),
        "find" => writeln!(
            writer,
            "'find <pattern>': Find lines containing a specific pattern."
        ),
        "top-words" => {
            writeln!(
                writer,
                "'top-words <count>': Print the top N most frequent words in the file."
            )
        }
        "filter-lines" => {
            writeln!(
                writer,
                "'filter-lines <pattern>': Filter lines containing a specific pattern."
            )
        }
        "replace" => writeln!(
            writer,
            "'replace <pattern> <replacement>': Replace occurrences of a pattern with a replacement."
        ),
        x => writeln!(writer, "Unsupported command: {}", x),
    }
}

/// Prints help text for every supported command.
#[allow(dead_code)]
pub fn print_help_all() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write_help_all(&mut handle).expect("failed to write command help");
}

/// Writes help text for every supported command.
pub fn write_help_all<W: io::Write>(writer: &mut W) -> io::Result<()> {
    writeln!(writer, "Available commands:")?;
    for cmd in [
        "stats",
        "longest-line",
        "unique-lines",
        "find",
        "top-words",
        "filter-lines",
        "replace",
    ] {
        write_help(cmd, writer)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command(cmd: &str, options: &[&str], flags: &[&str]) -> Result<Command, ArgsError> {
        Command::new(cmd, &options.to_vec(), &flags.to_vec())
    }

    #[test]
    fn new_builds_stats_command() {
        let command = command("stats", &[], &[]).unwrap();

        assert!(matches!(command.cmd, CommandType::Stats));
        assert!(!command.case_sensitive);
    }

    #[test]
    fn new_builds_longest_line_command() {
        let command = command("longest-line", &[], &[]).unwrap();

        assert!(matches!(command.cmd, CommandType::Longest));
        assert!(!command.case_sensitive);
    }

    #[test]
    fn new_builds_unique_lines_command() {
        let command = command("unique-lines", &[], &[]).unwrap();

        assert!(matches!(command.cmd, CommandType::Unique));
        assert!(!command.case_sensitive);
    }

    #[test]
    fn new_builds_find_command_with_case_sensitive_default() {
        let command = command("find", &["rust"], &[]).unwrap();

        assert!(matches!(command.cmd, CommandType::Find(pattern) if pattern == "rust"));
        assert!(command.case_sensitive);
    }

    #[test]
    fn new_builds_find_command_with_ignore_case_flag() {
        let command = command("find", &["rust"], &["--ignore-case"]).unwrap();

        assert!(matches!(command.cmd, CommandType::Find(pattern) if pattern == "rust"));
        assert!(!command.case_sensitive);
    }

    #[test]
    fn new_builds_top_words_command() {
        let command = command("top-words", &["5"], &[]).unwrap();

        assert!(matches!(command.cmd, CommandType::Top(count) if count == 5));
        assert!(!command.case_sensitive);
    }

    #[test]
    fn new_builds_filter_lines_command_with_ignore_case_flag() {
        let command = command("filter-lines", &["error"], &["--ignore-case"]).unwrap();

        assert!(matches!(command.cmd, CommandType::Filter(pattern) if pattern == "error"));
        assert!(!command.case_sensitive);
    }

    #[test]
    fn new_builds_replace_command() {
        let command = command("replace", &["old", "new"], &[]).unwrap();

        assert!(
            matches!(command.cmd, CommandType::Replace(pattern, replacement) if pattern == "old" && replacement == "new")
        );
        assert!(!command.case_sensitive);
    }

    #[test]
    fn new_rejects_unknown_command() {
        let error = command("missing", &[], &[]).unwrap_err();

        assert!(matches!(error, ArgsError::Invalid));
    }

    #[test]
    fn new_rejects_commands_missing_required_options() {
        for cmd in ["find", "top-words", "filter-lines", "replace"] {
            let error = command(cmd, &[], &[]).unwrap_err();

            assert!(matches!(error, ArgsError::MissingOption(command) if command.0 == cmd));
        }
    }

    #[test]
    fn new_rejects_replace_with_only_one_option() {
        let error = command("replace", &["old"], &[]).unwrap_err();

        assert!(matches!(error, ArgsError::MissingOption(command) if command.0 == "replace"));
    }

    #[test]
    fn new_rejects_invalid_top_words_count() {
        let error = command("top-words", &["five"], &[]).unwrap_err();

        assert!(matches!(error, ArgsError::InvalidOption(command, param)
                if command.0 == "top-words" && param.0.contains("invalid digit")));
    }
}
