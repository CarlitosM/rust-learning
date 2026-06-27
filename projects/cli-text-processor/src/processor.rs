use crate::commands::Command;
use crate::error::ProcessError;

use std::fs::read_to_string;
use std::io::{self, BufRead};

pub fn process_text(file_name: Option<String>, command: &Command) -> Result<String, ProcessError> {
    let text: Vec<String> = if let Some(file_name) = file_name {
        read_to_string(file_name)?
            .lines()
            .map(str::to_owned)
            .collect()
    } else {
        println!("Enter text (Ctrl+D to end):");
        io::stdin().lock().lines().collect::<Result<Vec<_>, _>>()?
    };

    Ok(text.join("\n"))
}
