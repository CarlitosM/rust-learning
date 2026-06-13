use std::env;
use std::io;

use crate::commands::{Command, write_help, write_help_all};
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
            handle_config_error(err);
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

fn handle_config_error(err: ArgsError) {
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();

    write_config_error(err, &mut stdout, &mut stderr).expect("failed to write config error");
}

fn write_config_error<W, E>(err: ArgsError, stdout: &mut W, stderr: &mut E) -> io::Result<()>
where
    W: io::Write,
    E: io::Write,
{
    writeln!(stderr, "Error: {:?}", err)?;
    writeln!(stdout, "Arguments pattern: <file> <command> [options]")?;
    match err {
        ArgsError::Invalid | ArgsError::NoArgs | ArgsError::MissingCmd => {
            write_help_all(stdout)?;
        }
        ArgsError::MissingOption(cmd) => {
            write_help(&cmd.0, stdout)?;
        }
        ArgsError::InvalidOption(cmd, _) => {
            write_help(&cmd.0, stdout)?;
        }
    }

    Ok(())
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

    fn config_error_output(err: ArgsError) -> (String, String) {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        write_config_error(err, &mut stdout, &mut stderr).unwrap();

        (
            String::from_utf8(stdout).unwrap(),
            String::from_utf8(stderr).unwrap(),
        )
    }

    #[test]
    fn handle_config_error_prints_all_help_for_general_argument_errors() {
        for err in [ArgsError::Invalid, ArgsError::NoArgs, ArgsError::MissingCmd] {
            let (stdout, stderr) = config_error_output(err);

            assert!(stdout.starts_with("Arguments pattern: <file> <command> [options]\n"));
            assert!(stdout.contains("Available commands:\n"));
            assert!(stdout.contains("'stats': Print general statistics about the file.\n"));
            assert!(
                stdout.contains(
                    "'replace <pattern> <replacement>': Replace occurrences of a pattern with a replacement.\n"
                )
            );
            assert!(stderr.starts_with("Error: "));
        }
    }

    #[test]
    fn handle_config_error_prints_command_help_for_missing_option() {
        let (stdout, stderr) =
            config_error_output(ArgsError::MissingOption(ValidCommand("find".to_string())));

        assert_eq!(
            stdout,
            "Arguments pattern: <file> <command> [options]\n\
             'find <pattern>': Find lines containing a specific pattern.\n"
        );
        assert_eq!(stderr, "Error: MissingOption(ValidCommand(\"find\"))\n");
    }

    #[test]
    fn handle_config_error_prints_command_help_for_invalid_option() {
        let (stdout, stderr) = config_error_output(ArgsError::InvalidOption(
            ValidCommand("top-words".to_string()),
            InvalidParam("invalid digit".to_string()),
        ));

        assert_eq!(
            stdout,
            "Arguments pattern: <file> <command> [options]\n\
             'top-words <count>': Print the top N most frequent words in the file.\n"
        );
        assert_eq!(
            stderr,
            "Error: InvalidOption(ValidCommand(\"top-words\"), InvalidParam(\"invalid digit\"))\n"
        );
    }
}
