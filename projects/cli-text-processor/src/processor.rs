use crate::commands::{Command, CommandType};
use crate::error::ProcessError;

use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self, BufRead};

pub fn process_text(file_name: Option<&str>, command: &Command) -> Result<String, ProcessError> {
    let text: Vec<String> = if let Some(file_name) = &file_name {
        read_to_string(file_name)?
            .lines()
            .map(|s| s.trim().to_owned())
            .collect()
    } else {
        println!("Enter text (Ctrl+D to end):");
        io::stdin().lock().lines().collect::<Result<Vec<_>, _>>()?
    };

    if text.is_empty() {
        return Err(ProcessError::NoText);
    }

    let result = match command.cmd {
        CommandType::Stats => {
            let stats_results = Stats::new(&text).print_results();
            if let Some(file_name) = &file_name {
                format!("File: {file_name}\n{stats_results}")
            } else {
                stats_results
            }
        }
        _ => todo!(),
    };

    Ok(result)
}

trait Results {
    fn print_results(&self) -> String;
}

#[derive(Debug)]
struct Stats {
    line_count: usize,
    word_count: usize,
    char_count: usize,
    unique_words: usize,
}

impl Stats {
    fn new(text: &[String]) -> Self {
        let mut word_count = 0;
        let mut char_count = 0;
        let mut distinct_words = HashSet::new();

        for line in text {
            line.split_whitespace().for_each(|word| {
                word_count += 1;
                char_count += word.chars().count();
                distinct_words.insert(word);
            });
        }

        Self {
            line_count: text.len(),
            word_count,
            char_count,
            unique_words: distinct_words.len(),
        }
    }
}

impl Results for Stats {
    fn print_results(&self) -> String {
        format!(
            "Lines: {}\nWords: {}\nCharacters: {}\nUnique words: {}",
            self.line_count, self.word_count, self.char_count, self.unique_words
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(lines: &[&str]) -> Vec<String> {
        lines.iter().map(|line| line.to_string()).collect()
    }

    #[test]
    fn new_counts_lines_words_characters_and_unique_words() {
        let input = text(&["Rust is fast", "Rust is memory-safe"]);

        let stats = Stats::new(&input);

        assert_eq!(stats.line_count, 2);
        assert_eq!(stats.word_count, 6);
        assert_eq!(stats.char_count, 27);
        assert_eq!(stats.unique_words, 4);
    }

    #[test]
    fn new_counts_blank_and_whitespace_only_lines_without_words() {
        let input = text(&["", "   ", "\t"]);

        let stats = Stats::new(&input);

        assert_eq!(stats.line_count, 3);
        assert_eq!(stats.word_count, 0);
        assert_eq!(stats.char_count, 0);
        assert_eq!(stats.unique_words, 0);
    }

    #[test]
    fn print_results_formats_stats_summary() {
        let stats = Stats {
            line_count: 2,
            word_count: 4,
            char_count: 19,
            unique_words: 4,
        };

        assert_eq!(
            stats.print_results(),
            "Lines: 2\nWords: 4\nCharacters: 19\nUnique words: 4"
        );
    }
}
