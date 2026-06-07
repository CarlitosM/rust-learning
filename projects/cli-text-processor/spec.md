# Week 7 — Mini-Project: CLI Text Processor

## Overview

Build a command-line tool that reads a text file and performs various
analysis and transformation operations on it. The tool should be well
structured, handle errors gracefully, and feel like something you'd
actually use.

## Usage

```
cargo run -p cli-text-processor -- <file> <command> [options]
```

Examples:
```
cargo run -p cli-text-processor -- input.txt stats
cargo run -p cli-text-processor -- input.txt find "the"
cargo run -p cli-text-processor -- input.txt top-words 5
cargo run -p cli-text-processor -- input.txt filter-lines "error"
cargo run -p cli-text-processor -- input.txt longest-line
```

## Commands to implement

### `stats`
Print general statistics about the file:
```
File: input.txt
Lines:      42
Words:      318
Characters: 1847
Unique words: 124
```

### `find <pattern>`
Find all lines containing the pattern (case-insensitive).
Print the line number and line content:
```
Line 3:  the quick brown fox
Line 17: another line with the word
```

### `top-words <n>`
Print the N most frequent words (case-insensitive, ignore punctuation):
```
Top 5 words:
  1. the        (14)
  2. and        (11)
  3. a          (8)
  4. to         (7)
  5. of         (6)
```

### `filter-lines <pattern>`
Print only lines that contain the pattern (no line numbers).

### `longest-line`
Print the longest line and its line number:
```
Longest line (line 23, 89 chars):
  "this is the longest line in the file..."
```

## Requirements

### Structure
Organise your code into modules — don't put everything in main.rs:
- `main.rs` — argument parsing and top-level error handling
- `processor.rs` — the core text processing logic
- `commands.rs` — one function or type per command
- `error.rs` — your custom error type

### Error handling
- Missing file: clear error message, exit with non-zero code
- Unknown command: list available commands
- Missing arguments (e.g. `find` with no pattern): helpful message
- Use a custom error enum, not String errors or unwrap()

### Code quality
- No unwrap() or expect() in non-main code (main can use them sparingly)
- Run clippy before submitting: `cargo clippy -- -W clippy::pedantic`
- All public functions should have doc comments (`///`)

### Stretch goals (optional, but good practice)
- `--ignore-case` flag for find and filter-lines (already case-insensitive
  for top-words, make the others configurable)
- `replace <pattern> <replacement>` command that prints the file with
  replacements made
- `unique-lines` command that prints lines that appear only once
- Accept stdin if no file is given (hint: `std::io::stdin()`)

## Suggested approach

Don't try to build everything at once. Suggested order:
1. Get `stats` working end to end first — file reading, basic counting,
   clean output. This forces you to set up the module structure and error
   handling before adding complexity.
2. Add `find` — introduces pattern matching across lines.
3. Add `top-words` — introduces the HashMap word frequency pattern.
4. Add `filter-lines` and `longest-line` — straightforward once you have
   the infrastructure.
5. Polish: error messages, clippy, doc comments.

## Test file

Create a file `input.txt` in the project root to test with.
Something with enough content to make `top-words` interesting —
a few paragraphs of text, maybe 30-50 lines.
You can use any text you like (a Wikipedia article paste works well).

## What I'll look at when you submit

- Module structure — is the code organised sensibly?
- Error handling — are all error cases covered with a custom type?
- Idiom — are you using iterators, pattern matching, and traits naturally?
- Clippy — does it pass with `-W clippy::pedantic`?
- The things that are hardest to get right: argument parsing without
  a library (just std::env::args()), and the word frequency map.

Good luck — this is the first thing you'll build that feels like real Rust.
