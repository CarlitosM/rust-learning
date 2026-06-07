// Week 5 — Error Handling
// =======================
// Prerequisites: weeks 1–4.
//
// This week's theme: errors are values. A function that can fail returns
// Result<T, E> — the compiler forces you to handle both cases. No hidden
// exceptions, no runtime surprises.
//
// The JS comparison: like Promise<T> but synchronous and checked at compile
// time. The ? operator is like await — it unwraps the happy path and
// propagates the sad path automatically.
//
// Run with: cargo run -p week-05-errors

fn main() {
    section_1_result_basics();
    section_2_question_mark();
    section_3_custom_errors();
    section_4_error_conversion();
    section_5_putting_together();
}

// ─── SECTION 1: Result basics ────────────────────────────────────────────────
//
// Result<T, E> is an enum with two variants:
//   Ok(T)  — success, contains the value
//   Err(E) — failure, contains the error
//
// You must handle both — the compiler won't let you ignore an Err.

use std::{
    fmt,
    num::{IntErrorKind, ParseFloatError, ParseIntError},
};

fn section_1_result_basics() {
    // --- 1a: matching on Result ---
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("division by zero"))
        } else {
            Ok(a / b)
        }
    }

    // TODO: call divide(10.0, 2.0) and divide(10.0, 0.0)
    // Handle both cases with a match expression — print the result or the error
    // Your code:
    match divide(10.0, 2.0) {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {}", e),
    }

    // --- 1b: Result combinators ---
    // Just like Option, Result has methods that let you chain operations
    // without explicit match expressions.
    //
    // .map(|v| ...)        — transform Ok value, pass Err through
    // .map_err(|e| ...)    — transform Err value, pass Ok through
    // .unwrap_or(default)  — get Ok value or a default on Err
    // .unwrap_or_else(|e| ...) — get Ok value or compute from error
    // .and_then(|v| ...)   — chain another fallible operation (flatMap)

    let result = divide(10.0, 2.0)
        .map(|v| v * 2.0) // double the result if Ok
        .map_err(|e| format!("Error: {}", e)); // wrap error message if Err
    println!("{:?}", result);

    // TODO: chain divide(10.0, 3.0) with:
    //   1. map: round to 2 decimal places  (hint: (v * 100.0).round() / 100.0)
    //   2. and_then: fail if result > 5.0 with Err("result too large")
    //   3. unwrap_or: fall back to 0.0
    // Print the final value
    // Your code:
    println!(
        "{}",
        divide(10.0, 3.0)
            .map(|res| (res * 100.0).round() / 100.0)
            .and_then(|res| if res > 5.0 {
                Err("result too large".to_string())
            } else {
                Ok(res)
            })
            .unwrap_or(0.0)
    );

    // --- 1c: collecting Results ---
    // If you have a Vec<Result<T, E>>, you can collect into Result<Vec<T>, E>
    // It succeeds if all elements are Ok, fails on the first Err.
    let strings = vec!["1", "2", "three", "4"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("parsed: {:?}", numbers); // should be Err

    let valid = vec!["1", "2", "3"];
    let numbers: Result<Vec<i32>, _> = valid.iter().map(|s| s.parse::<i32>()).collect();
    println!("parsed: {:?}", numbers); // should be Ok([1, 2, 3])

    // Q: what does _ mean in Result<Vec<i32>, _>?
    // Your answer:
    // _ is used to indicate an unused value, not sure what that means in regards to a type, maybe it means that we don't what the specifc type the error is?
}

// ─── SECTION 2: The ? operator ───────────────────────────────────────────────
//
// ? is syntactic sugar for: if Err, return it; if Ok, unwrap it.
// It replaces a lot of match boilerplate and makes fallible code
// read almost like happy-path code.
//
// It only works in functions that return Result (or Option).

fn section_2_question_mark() {
    // --- 2a: without ? (verbose) ---
    fn parse_and_double_verbose(s: &str) -> Result<i32, ParseIntError> {
        let n = match s.parse::<i32>() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        Ok(n * 2)
    }

    // --- 2b: with ? (clean) ---
    fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
        // TODO: rewrite parse_and_double_verbose using ?
        let n = s.parse::<i32>()?;
        Ok(n * 2)
    }

    println!("{:?}", parse_and_double("5"));
    println!("{:?}", parse_and_double("abc"));

    // --- 2c: chaining ? ---
    fn parse_sum(a: &str, b: &str) -> Result<i32, ParseIntError> {
        // TODO: parse both strings and return their sum
        // Use ? on each parse — if either fails, the error propagates
        let x = a.parse::<i32>()?;
        let y = b.parse::<i32>()?;
        Ok(x + y)
    }

    println!("{:?}", parse_sum("3", "4"));
    println!("{:?}", parse_sum("3", "abc"));

    // --- 2d: ? in a chain ---
    // Sometimes you want to use ? in the middle of a chain.
    // You can by wrapping in a closure that returns Result:
    fn process(input: &str) -> Result<String, ParseIntError> {
        // TODO: parse input as i32, multiply by 3, convert to String
        // Use ? to propagate parse errors
        // return format: "result: 15" for input "5"
        let n = input.parse::<i32>()?;
        Ok(format!("result: {}", n * 3))
    }

    println!("{:?}", process("5"));
    println!("{:?}", process("bad"));

    // Q: what would happen if you used ? in a function returning ()?
    // Try it mentally — what would the compiler say?
    // Your answer:
    // I belive the compiler would say something along the lines of "expected `Result`, found `()`" since the ? will return a Result (or Option) type if it encounters an error, not the () type
}

// ─── SECTION 3: Custom error types ───────────────────────────────────────────
//
// String errors work for quick scripts but aren't great for libraries or
// larger programs — callers can't match on them programmatically.
// Custom error types let callers handle specific errors differently.
//
// The standard pattern: an enum of error variants + impl std::error::Error

fn section_3_custom_errors() {
    // --- 3a: define and use a custom error ---
    // TODO: implement ParseAgeError below, then use it here

    let valid = parse_age("25");
    let negative = parse_age("-5");
    let too_old = parse_age("200");
    let not_a_number = parse_age("abc");

    println!("{:?}", valid);
    println!("{:?}", negative);
    println!("{:?}", too_old);
    println!("{:?}", not_a_number);

    // --- 3b: matching on specific variants ---
    // Because ParseAgeError is an enum, callers can handle each case:
    match parse_age("abc") {
        Ok(age) => println!("age: {}", age),
        Err(ParseAgeError::NotANumber(_)) => println!("please enter a number"),
        Err(ParseAgeError::TooYoung) => println!("must be non-negative"),
        Err(ParseAgeError::TooOld) => println!("unrealistic age"),
    }
}

// TODO: define ParseAgeError with three variants:
//   - NotANumber(std::num::ParseIntError)  — wraps the parse error
//   - TooYoung                             — age was negative
//   - TooOld                              — age was > 150
//
// Derive Debug on it.
// Implement fmt::Display for it (human-readable error messages).
// Implement std::error::Error for it (empty impl is fine — it has defaults).

#[derive(Debug)]
enum ParseAgeError {
    NotANumber(ParseIntError),
    TooYoung,
    TooOld,
}

impl std::error::Error for ParseAgeError {}

impl fmt::Display for ParseAgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: match on self and write a human-readable message for each variant
        // e.g. "invalid number: {}" for NotANumber, "age cannot be negative" for TooYoung
        match self {
            ParseAgeError::NotANumber(e) => write!(f, "invalid number: {}", e),
            ParseAgeError::TooYoung => write!(f, "age cannot be negative"),
            ParseAgeError::TooOld => write!(f, "age cannot be greater than 150"),
        }
    }
}

// TODO: implement parse_age
// - parse the string as i32 (map the ParseIntError to NotANumber)
// - return TooYoung if age < 0
// - return TooOld if age > 150
// - return Ok(age as u8) if valid
fn parse_age(s: &str) -> Result<u8, ParseAgeError> {
    let n = s.parse::<u8>().map_err(|e| match e.kind() {
        IntErrorKind::NegOverflow => ParseAgeError::TooYoung,
        _ => ParseAgeError::NotANumber(e),
    })?;

    if n > 150 {
        Err(ParseAgeError::TooOld)
    } else {
        Ok(n)
    }
}

// ─── SECTION 4: Error conversion & Box<dyn Error> ────────────────────────────
//
// Real programs mix errors from different sources — your own types, std,
// third-party crates. Two tools help:
//
// 1. From trait: implement From<OtherError> for YourError, and ? will
//    automatically convert between them.
//
// 2. Box<dyn std::error::Error>: a trait object that can hold any error.
//    Quick and flexible, but loses type information.

fn section_4_error_conversion() {
    // --- 4a: From conversion ---
    // If you implement From<ParseIntError> for ParseAgeError,
    // ? will automatically convert ParseIntError into ParseAgeError.

    // TODO: implement From<ParseIntError> for ParseAgeError
    // (maps to ParseAgeError::NotANumber)
    // Then rewrite parse_age using ? instead of .map_err()

    // --- 4b: Box<dyn Error> for mixed errors ---
    // When you're mixing error types and don't need to match on them,
    // Box<dyn Error> is a convenient escape hatch.
    // A type alias makes it less verbose:
    type BoxError = Box<dyn std::error::Error>;

    fn read_and_parse(s: &str, factor: &str) -> Result<i32, BoxError> {
        // TODO: parse s as i32, parse factor as i32, return their product
        // Both parses can fail with different error messages — use ?
        // The compiler will box them automatically when returning
        let n = s.parse::<i32>()?;
        let factor = factor.parse::<i32>()?;
        Ok(n * factor)
    }

    println!("{:?}", read_and_parse("6", "7"));
    println!("{:?}", read_and_parse("abc", "7"));
    println!("{:?}", read_and_parse("6", "xyz"));

    // Q: what do you lose when using Box<dyn Error> vs a custom enum?
    // Your answer:
    // compiler type information

    // Q: when would Box<dyn Error> be the right choice?
    // Your answer:
    // when you can't know the error type at compile time
}

// ─── SECTION 5: Putting it together — a CSV row parser ───────────────────────
//
// Parse rows from a simple CSV format: "name,age,score"
// Each field can fail in different ways — use a custom error type,
// From conversions, and ? to keep the parsing code clean.
//
// Example input:  "Alice,30,95.5"
// Example output: Person { name: "Alice", age: 30, score: 95.5 }

fn section_5_putting_together() {
    let rows = vec![
        "Alice,30,95.5",
        "Bob,25,87.0",
        "Charlie,abc,70.0",   // bad age
        "Diana,30,not_a_num", // bad score
        "Eve",                // missing fields
        "Frank,200,80.0",     // age too old
        "Grace,-1,90.0",      // age too young
    ];

    for row in &rows {
        match parse_row(row) {
            Ok(person) => println!("ok: {:?}", person),
            Err(e) => println!("err [{}]: {}", row, e),
        }
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    score: f64,
}

// TODO: define CsvError with variants:
//   - MissingField(&'static str)   — field name that's missing
//   - InvalidAge(ParseAgeError)    — wraps your age error from section 3
//   - InvalidScore(std::num::ParseFloatError) — bad score value
//
// Implement Display and Error for it.
// Implement From<ParseAgeError> for CsvError.
// Implement From<std::num::ParseFloatError> for CsvError.

#[derive(Debug)]
enum CsvError {
    MissingField(String),
    InvalidAge(ParseAgeError),
    InvalidScore(ParseFloatError),
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvError::MissingField(field) => write!(f, "missing field: {}", field),
            CsvError::InvalidAge(age) => write!(f, "invalid age: {}", age),
            CsvError::InvalidScore(score) => write!(f, "invalid score: {}", score),
        }
    }
}

impl std::error::Error for CsvError {}

impl From<ParseAgeError> for CsvError {
    fn from(age: ParseAgeError) -> Self {
        CsvError::InvalidAge(age)
    }
}

impl From<ParseFloatError> for CsvError {
    fn from(score: ParseFloatError) -> Self {
        CsvError::InvalidScore(score)
    }
}

// TODO: implement parse_row
// - split on ','
// - extract name, age string, score string (return MissingField if absent)
// - parse age using parse_age() — ? will convert ParseAgeError to CsvError
// - parse score as f64 — ? will convert ParseFloatError to CsvError
// - return Ok(Person { ... })
fn parse_row(row: &str) -> Result<Person, CsvError> {
    let mut fields = row.split(',');

    vec![0, 1, 2].iter().try_fold(
        Person {
            name: String::new(),
            age: 0,
            score: 0.,
        },
        |mut p, i| {
            let field = fields.nth(*i as usize).unwrap_or("");
            let missing = field.is_empty();

            match i {
                0 if !missing => p.name = field.to_string(),
                0 if missing => return Err(CsvError::MissingField("name".to_string())),
                1 if !missing => p.age = parse_age(field)?,
                1 if missing => return Err(CsvError::MissingField("age".to_string())),
                2 if !missing => p.score = field.parse()?,
                2 if missing => return Err(CsvError::MissingField("score".to_string())),
                _ => return Err(CsvError::MissingField("unknown".to_string())),
            }
            Ok(p)
        },
    )
}

// ─── REFLECTION ───────────────────────────────────────────────────────────────
//
// 1. What does the ? operator actually do? Write it out as a match expression.
//    match this_is_a_result_type {
//        Ok(v) => v,
//        Err(e) => return Err(e),
//    }
//
// 2. What's the difference between unwrap() and ?
//    When is each appropriate?
//    unwrap() panics on error, and is appropriate to use when the error would be unrecoverable, prototying, or you are sure that an error can't occur.
//    ? will propagate an error or return the resulting value, and is appropriate to use when the error can be propagated up the call stack and handled by the caller.
//
// 3. Why define a custom error enum instead of using String errors?
//    String errors are less type safe and can't be matched on, also custom errors provide the opportunity to be more descriptive and provide context about the error.
//
// 4. What does implementing From<X> for MyError give you?
//    it allows errors of x type to be converted into MyError type implicitly, so you don't need to handle the conversion every time.
//
// 5. When would you choose Box<dyn Error> over a custom error enum?
//    Box<dyn Error> is appropriate when the error type is not known at compile time
//
// 6. What was the hardest part of this week?
//    getting used to the implicit type conversions that happen when using the ? operator or when propagating errors from an iterator
