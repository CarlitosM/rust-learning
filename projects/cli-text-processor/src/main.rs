use std::env;

use crate::commands::{Command, get_help, get_help_all};
use crate::error::ArgsError;

mod commands;
mod error;
mod processor;

fn main() {
    let args = self::env::args().skip(1).collect::<Vec<String>>();
    let config = Config::new(&args);

    match config {
        Ok(config) => {
            todo!()
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            println!("{}", get_config_help_msg(err));
        }
    }
}

#[derive(Debug)]
struct Config {
    command: Command,
    file_name: Option<String>,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Self, ArgsError> {
        let args_qty = args.len();

        if args_qty == 0 {
            Err(ArgsError::NoArgs)
        } else {
            let (file_name, cmd, params, flags) = args.iter().enumerate().fold(
                Ok((None, "", Vec::new(), Vec::new())),
                |acc: Result<(Option<String>, &str, Vec<&str>, Vec<&str>), ArgsError>, (i, arg)| {
                    match acc {
                        Ok(mut val) => {
                            let is_file = arg.ends_with(".txt");
                            let is_flag = arg.starts_with("--");

                            match i {
                                0 => {
                                    if is_file && args_qty == 1 {
                                        return Err(ArgsError::MissingCmd);
                                    }

                                    if is_file && args_qty > 1 {
                                        val.0 = Some(arg.clone());
                                    }

                                    if !is_file {
                                        val.1 = arg;
                                    }
                                }
                                1 => {
                                    if is_file {
                                        return Err(ArgsError::Invalid);
                                    }

                                    let have_file = val.0.is_some();

                                    if have_file {
                                        val.1 = arg;
                                    }

                                    if !have_file && !is_flag {
                                        val.2.push(arg);
                                    }

                                    if !have_file && is_flag {
                                        val.3.push(arg);
                                    }
                                }
                                _ => {
                                    if is_file {
                                        return Err(ArgsError::Invalid);
                                    }

                                    if !is_flag {
                                        val.2.push(arg);
                                    } else {
                                        val.3.push(arg);
                                    }
                                }
                            }
                            Ok(val)
                        }
                        Err(e) => Err(e),
                    }
                },
            )?;

            let command = Command::new(cmd, &params, &flags)?;
            Ok(Config { command, file_name })
        }
    }
}

fn get_config_help_msg(err: ArgsError) -> String {
    let mut msg = String::from("Arguments pattern: <file> <command> [options]\n");

    match err {
        ArgsError::Invalid | ArgsError::NoArgs | ArgsError::MissingCmd => {
            msg.push_str(&get_help_all());
        }
        ArgsError::MissingOption(cmd) => {
            msg.push_str(&get_help(&cmd.0));
        }
        ArgsError::InvalidOption(cmd, _) => {
            msg.push_str(&get_help(&cmd.0));
        }
    }

    msg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::CommandType;
    use crate::error::{ArgsError, InvalidParam, ValidCommand};

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn new_builds_config_from_command_only_args() {
        let config = Config::new(&args(&["stats"])).unwrap();

        assert!(config.file_name.is_none());
        assert!(matches!(config.command.cmd, CommandType::Stats));
        assert!(!config.command.case_sensitive);
    }

    #[test]
    fn new_accepts_file_name_before_command() {
        let config = Config::new(&args(&["input.txt", "longest-line"])).unwrap();

        assert_eq!(config.file_name, Some("input.txt".to_string()));
        assert!(matches!(config.command.cmd, CommandType::Longest));
    }

    #[test]
    fn new_passes_options_and_flags_to_command_parser() {
        let config = Config::new(&args(&["find", "Rust", "--ignore-case"])).unwrap();

        assert!(config.file_name.is_none());
        assert!(!config.command.case_sensitive);
        assert!(matches!(config.command.cmd, CommandType::Find(pattern) if pattern == "Rust"));
    }

    #[test]
    fn new_rejects_file_name_after_command() {
        let error = Config::new(&args(&["stats", "input.txt"])).unwrap_err();

        assert!(matches!(error, ArgsError::Invalid));
    }

    #[test]
    fn new_returns_command_parser_errors() {
        let error = Config::new(&args(&["find"])).unwrap_err();

        assert!(matches!(error, ArgsError::MissingOption(command) if command.0 == "find"));
    }

    #[test]
    fn new_rejects_empty_args() {
        let error = Config::new(&args(&[])).unwrap_err();

        assert!(matches!(error, ArgsError::NoArgs));
    }

    #[test]
    fn new_rejects_file_name_without_command() {
        let error = Config::new(&args(&["input.txt"])).unwrap_err();

        assert!(matches!(error, ArgsError::MissingCmd));
    }

    #[test]
    fn get_config_help_msg_returns_all_help_for_general_argument_errors() {
        for err in [ArgsError::Invalid, ArgsError::NoArgs, ArgsError::MissingCmd] {
            let msg = get_config_help_msg(err);

            assert!(msg.starts_with("Arguments pattern: <file> <command> [options]\n"));
            assert!(msg.contains("Available commands:\n"));
            assert!(msg.contains("'stats': Print general statistics about the file.\n"));
            assert!(
                msg.contains(
                    "'replace <pattern> <replacement>': Replace occurrences of a pattern with a replacement.\n"
                )
            );
        }
    }

    #[test]
    fn get_config_help_msg_returns_command_help_for_missing_option() {
        let msg = get_config_help_msg(ArgsError::MissingOption(ValidCommand("find".to_string())));

        assert_eq!(
            msg,
            "Arguments pattern: <file> <command> [options]\n\
             'find <pattern>': Find lines containing a specific pattern."
        );
    }

    #[test]
    fn get_config_help_msg_returns_command_help_for_invalid_option() {
        let msg = get_config_help_msg(ArgsError::InvalidOption(
            ValidCommand("top-words".to_string()),
            InvalidParam("invalid digit".to_string()),
        ));

        assert_eq!(
            msg,
            "Arguments pattern: <file> <command> [options]\n\
             'top-words <count>': Print the top N most frequent words in the file."
        );
    }
}
