// Week 6 — Enums & Pattern Matching in Depth
// ===========================================
// Prerequisites: weeks 1–5.
//
// This week's theme: enums are not just error types — they're the primary
// tool for modelling data that can be in one of several states. Combined
// with pattern matching, they let you write exhaustive, compiler-checked
// logic with no runtime surprises.
//
// The JS comparison: like a discriminated union in TypeScript, but the
// compiler enforces exhaustiveness everywhere — no forgotten cases.
//
// Run with: cargo run -p week-06-enums
use std::fmt;

fn main() {
    section_1_enum_data();
    section_2_pattern_matching();
    section_3_option_in_depth();
    section_4_newtype_pattern();
    section_5_putting_together();
}

// ─── SECTION 1: Enums that carry data ────────────────────────────────────────
//
// Rust enums can carry different data in each variant — more like algebraic
// data types than C-style enums. Each variant can have its own shape.
//
// This is the foundation of modelling "a thing that is one of several kinds".

fn section_1_enum_data() {
    // --- 1a: variants with different data shapes ---
    #[derive(Debug)]
    enum Shape {
        Circle(f64),                         // radius
        Rectangle(f64, f64),                 // width, height
        Triangle { base: f64, height: f64 }, // named fields
    }

    // TODO: implement area() as a method on Shape using impl
    // Circle: PI * r^2,  Rectangle: w * h,  Triangle: 0.5 * b * h
    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Circle(r) => std::f64::consts::PI * r * r,
                Shape::Rectangle(w, h) => w * h,
                Shape::Triangle { base, height } => 0.5 * base * height,
            }
        }

        fn name(&self) -> &str {
            match self {
                Shape::Circle(_) => "Circle",
                Shape::Rectangle(_, _) => "Rectangle",
                Shape::Triangle { .. } => "Triangle",
            }
        }
    }

    let shapes = vec![
        Shape::Circle(3.0),
        Shape::Rectangle(4.0, 5.0),
        Shape::Triangle {
            base: 6.0,
            height: 4.0,
        },
    ];

    for shape in &shapes {
        println!("{}: {:.2}", shape.name(), shape.area());
    }

    // --- 1b: enums as state machines ---
    // Enums are perfect for representing state — each variant IS a state,
    // and the data it carries is only available in that state.
    #[derive(Debug)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLight {
        fn next(&self) -> TrafficLight {
            match self {
                TrafficLight::Green => TrafficLight::Yellow,
                TrafficLight::Yellow => TrafficLight::Red,
                TrafficLight::Red => TrafficLight::Green,
            }
        }

        fn duration_secs(&self) -> u32 {
            match self {
                TrafficLight::Green => 45,
                TrafficLight::Yellow => 5,
                TrafficLight::Red => 60,
            }
        }

        fn can_go(&self) -> bool {
            match self {
                TrafficLight::Green => true,
                _ => false,
            }
        }
    }

    let mut light = TrafficLight::Red;
    for _ in 0..4 {
        println!(
            "{:?} — {}s — go: {}",
            light,
            light.duration_secs(),
            light.can_go()
        );
        light = light.next();
    }

    // Q: why is an enum better than constants (const RED: u8 = 0) for this?
    // Your answer: you can pattern match exhaustively on an enum, but not on a constant of a primitive type, also the enum better encapsulates the possible traffic light states
}

// ─── SECTION 2: Pattern matching in depth ────────────────────────────────────
//
// match is exhaustive — the compiler errors if you miss a case.
// But patterns go further: guards, bindings, nested patterns, ranges.

fn section_2_pattern_matching() {
    // --- 2a: match guards ---
    let numbers = vec![1i32, -3, 0, 7, -1, 42, -100];
    for &n in &numbers {
        // TODO: match n with guards:
        //   negative numbers: print "negative: {n}"
        //   zero:             print "zero"
        //   1..=9:            print "small positive: {n}"
        //   other positive:   print "large positive: {n}"
        match n {
            n if n < 0 => println!("negative: {n}"),
            0 => println!("zero"),
            1..=9 => println!("small positive: {n}"),
            _ => println!("large positive: {n}"),
        }
    }

    // --- 2b: destructuring in patterns ---
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[derive(Debug)]
    enum Command {
        Move(Point),
        Turn(f64), // degrees
        Print(String),
        Quit,
    }

    let commands = vec![
        Command::Move(Point { x: 1.0, y: 2.0 }),
        Command::Turn(90.0),
        Command::Print(String::from("hello")),
        Command::Quit,
    ];

    // TODO: match each command and print a description:
    //   Move: "move to (1.0, 2.0)"
    //   Turn: "turn 90.0 degrees"
    //   Print: "print: hello"
    //   Quit: "quit"
    for cmd in &commands {
        match cmd {
            Command::Move(Point { x, y }) => println!("move to ({}, {})", x, y),
            Command::Turn(deg) => println!("turn {} degrees", deg),
            Command::Print(s) => println!("print: {}", s),
            Command::Quit => println!("quit"),
        }
    }

    // --- 2c: if let and while let ---
    // For when you only care about one variant

    let values: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];

    // TODO: use if let to print only the Some values
    for v in &values {
        if let Some(n) = v {
            println!("{n}");
        }
    }

    // TODO: use a while let to pop from a stack until empty
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(n) = stack.pop() {
        println!("{n}");
    }

    // --- 2d: nested pattern matching ---
    #[derive(Debug)]
    enum Response {
        Ok(Option<String>), // success, maybe with a body
        Err { code: u16, message: String },
    }

    let responses = vec![
        Response::Ok(Some(String::from("data"))),
        Response::Ok(None),
        Response::Err {
            code: 404,
            message: String::from("not found"),
        },
        Response::Err {
            code: 500,
            message: String::from("server error"),
        },
    ];

    // TODO: match each response:
    //   Ok(Some(body)): "success: {body}"
    //   Ok(None):       "success: empty"
    //   Err 404:        "not found"
    //   Err other:      "error {code}: {message}"
    for r in &responses {
        match r {
            Response::Ok(Some(body)) => println!("success: {body}"),
            Response::Ok(None) => println!("success: empty"),
            Response::Err { code: 404, .. } => println!("not found"),
            Response::Err { code, message } => println!("error {code}: {message}"),
        }
    }

    // Q: what does the compiler do if you remove one arm from a match?
    // Your answer: it tells you that you have not handled all possible cases (non-exhaustive) and which cases are missing
}

// ─── SECTION 3: Option in depth ──────────────────────────────────────────────
//
// You've used Option — now let's use it as a design tool.
// The goal: represent optional data without null, and chain operations
// on it without explicit null checks.

fn section_3_option_in_depth() {
    // --- 3a: Option combinators ---
    // .map()          — transform Some value, pass None through
    // .and_then()     — chain Option-returning operation (flatMap)
    // .or_else()      — provide fallback if None
    // .filter()       — Some becomes None if predicate fails
    // .unwrap_or()    — get value or default
    // .ok_or()        — convert Option to Result

    fn find_user(id: u32) -> Option<String> {
        match id {
            1 => Some(String::from("Alice")),
            2 => Some(String::from("Bob")),
            _ => None,
        }
    }

    fn find_email(name: &str) -> Option<String> {
        match name {
            "Alice" => Some(String::from("alice@example.com")),
            _ => None,
        }
    }

    // TODO: chain find_user and find_email using combinators (no match/if let)
    // For id 1: should find email
    // For id 2: should get None (Bob has no email)
    // For id 99: should get None (user not found)
    // Print all three results using unwrap_or("no email found")
    for id in [1, 2, 99] {
        println!(
            "{}",
            find_user(id)
                .and_then(|name| find_email(name.as_str()))
                .unwrap_or("no email found".to_string())
        );
    }

    // --- 3b: Option as a design tool ---
    // Instead of returning a sentinel value (-1, "", etc.),
    // return Option. The caller is forced to handle the absent case.

    // TODO: implement these functions returning Option
    fn first_even(numbers: &[i32]) -> Option<i32> {
        numbers.iter().find(|n| *n % 2 == 0).copied()
    }

    fn parse_positive(s: &str) -> Option<u32> {
        s.parse::<u32>().ok().filter(|n| *n > 0)
    }

    println!("{:?}", first_even(&[1, 3, 4, 6]));
    println!("{:?}", first_even(&[1, 3, 5]));
    println!("{:?}", parse_positive("42"));
    println!("{:?}", parse_positive("0"));
    println!("{:?}", parse_positive("abc"));

    // --- 3c: ? with Option ---
    // ? works on Option too — returns None early if the value is None

    fn initials(full_name: &str) -> Option<String> {
        // TODO: get the first character of the first and last word
        // "Alice Smith" -> "A.S."
        // Return None if the name has fewer than 2 words
        // Hint: split_whitespace, next(), last(), chars().next()
        let mut names = full_name.split_whitespace();

        let first_letter = names.next()?.chars().next()?;
        let second_letter = names.next()?.chars().next()?;

        Some(format!(
            "{}.{}.",
            first_letter.to_uppercase(),
            second_letter.to_uppercase()
        ))
    }

    println!("{:?}", initials("Alice Smith"));
    println!("{:?}", initials("Madonna"));
    println!("{:?}", initials("Mary Jane Watson"));

    // Q: why is Option<T> better than returning a sentinel value like -1 or ""?
    // Your answer: because the complire will require that both cases of the option are handled,
    // where with the sentinal value, the caller will need to know that there is a sentinal (a null or None case) that can happen
    // and will need to remeber to handle that case, as well as knowing what the sentinal value is that signals that case
}

// ─── SECTION 4: The newtype pattern ──────────────────────────────────────────
//
// A newtype is a tuple struct with one field. It wraps an existing type
// to give it a distinct identity — the compiler treats them as different types
// even if the underlying representation is the same.
//
// This is the correct version of your u8-for-age instinct from week 5.

fn section_4_newtype_pattern() {
    // --- 4a: basic newtype ---
    // Without newtypes, this compiles — but it's wrong:
    fn set_dimensions_bad(width: u32, height: u32) {
        let _ = (width, height);
    }
    // set_dimensions_bad(height, width); // oops — swapped, but compiles

    // With newtypes, the compiler catches the swap:
    #[derive(Debug, Clone, Copy)]
    struct Width(u32);
    #[derive(Debug, Clone, Copy)]
    struct Height(u32);

    fn set_dimensions(width: Width, height: Height) {
        println!("{}x{}", width.0, height.0);
    }

    let w = Width(800);
    let h = Height(600);
    set_dimensions(w, h);
    // set_dimensions(h, w); // uncomment — compiler catches the swap!

    // --- 4b: newtype with validation ---
    // This is the right version of the age-as-u8 idea from week 5.
    // The newtype is only constructible through a validated constructor.

    #[derive(Debug, Clone, Copy)]
    struct Age(u8);

    impl Age {
        // TODO: implement new() -> Result<Age, String>
        // valid range: 0..=150
        fn new(value: u8) -> Result<Age, String> {
            if (..=150u8).contains(&value) {
                Ok(Age(value))
            } else {
                Err("invalide age".to_string())
            }
        }

        fn value(&self) -> u8 {
            self.0
        }
    }

    println!("{:?}", Age::new(25));
    println!("{:?}", Age::new(200)); // u8 max is 255, so this is representable but invalid
    println!("{:?}", Age::new(0));

    // --- 4c: newtype for unit safety ---
    // Classic bug: mixing up units. Newtypes prevent it at compile time.
    #[derive(Debug, Clone, Copy)]
    struct Metres(f64);
    #[derive(Debug, Clone, Copy)]
    struct Kilograms(f64);

    // TODO: implement Add for Metres so you can do metres + metres
    use std::ops::Add;
    impl Add for Metres {
        type Output = Metres;
        fn add(self, other: Metres) -> Metres {
            Metres(self.0 + other.0)
        }
    }

    let a = Metres(1.5);
    let b = Metres(2.5);
    println!("{:?}", a + b);
    // let wrong = a + Kilograms(1.0); // uncomment — compiler catches unit mismatch!

    // Q: what problem does the newtype pattern solve that a type alias doesn't?
    // (hint: type Metres = f64 — what can you still do wrong with this?)
    // Your answer: the newtype pattern creates a literaly new type, which means full type checking against the new type not the underlying type
    // a type alias still has type checking, just against the type that it's aliasing
}

// ─── SECTION 5: Putting it together — a simple expression evaluator ──────────
//
// Model a mathematical expression as an enum, then evaluate it.
// This is the classic use case for recursive enums — a tree structure
// where each node is either a leaf (number) or an operation on sub-expressions.
//
// Expr::Num(2.0)
// Expr::Add(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(2.0)))  -> 3.0
// Expr::Mul(Box::new(Expr::Num(2.0)), Box::new(Expr::Add(...)))  -> nested

fn section_5_putting_together() {
    // Build: (2 + 3) * (10 - 4)  should equal 30
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Num(2.0)),
            Box::new(Expr::Num(3.0)),
        )),
        Box::new(Expr::Sub(
            Box::new(Expr::Num(10.0)),
            Box::new(Expr::Num(4.0)),
        )),
    );

    println!("result: {}", expr.eval()); // 30
    println!("display: {}", expr); // "(2 + 3) * (10 - 4)"

    // Division by zero should return an error:
    let div_zero = Expr::Div(Box::new(Expr::Num(5.0)), Box::new(Expr::Num(0.0)));
    println!("div zero: {:?}", div_zero.eval_safe()); // Err(...)

    // Nested:
    let nested = Expr::Add(
        Box::new(Expr::Mul(
            Box::new(Expr::Num(3.0)),
            Box::new(Expr::Num(4.0)),
        )),
        Box::new(Expr::Div(
            Box::new(Expr::Num(10.0)),
            Box::new(Expr::Num(2.0)),
        )),
    );
    println!("nested: {}", nested.eval()); // 17
}

// TODO: define Expr enum with variants:
//   Num(f64)
//   Add(Box<Expr>, Box<Expr>)
//   Sub(Box<Expr>, Box<Expr>)
//   Mul(Box<Expr>, Box<Expr>)
//   Div(Box<Expr>, Box<Expr>)
//
// Why Box? Because recursive enums need indirection — the compiler needs
// to know the size of Expr at compile time, but Expr containing Expr
// would be infinitely sized without the heap indirection Box provides.

#[derive(Debug)]
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    // TODO: eval(&self) -> f64
    // Recursively evaluate the expression.
    // For Div, treat division by zero as 0.0 (handle properly in eval_safe)
    fn eval(&self) -> f64 {
        match self {
            Expr::Num(n) => *n,
            Expr::Add(a, b) => a.eval() + b.eval(),
            Expr::Sub(a, b) => a.eval() - b.eval(),
            Expr::Mul(a, b) => a.eval() * b.eval(),
            Expr::Div(a, b) => {
                let divisor = b.eval();
                if divisor == 0.0 {
                    0.0
                } else {
                    a.eval() / divisor
                }
            }
        }
    }

    // TODO: eval_safe(&self) -> Result<f64, String>
    // Like eval, but returns Err("division by zero") instead of 0.0
    fn eval_safe(&self) -> Result<f64, String> {
        match self {
            Expr::Div(_, b) => match **b {
                Expr::Num(n) if n == 0. => Err("division by zero".to_string()),
                _ => Ok(self.eval()),
            },
            _ => Ok(self.eval()),
        }
    }
}

// TODO: implement fmt::Display for Expr
// Num(2.0):           "2"
// Add(left, right):   "(left + right)"
// Sub(left, right):   "(left - right)"
// Mul(left, right):   "(left * right)"
// Div(left, right):   "(left / right)"
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Add(a, b) => write!(f, "({} + {})", a, b),
            Expr::Sub(a, b) => write!(f, "({} - {})", a, b),
            Expr::Mul(a, b) => write!(f, "({} * {})", a, b),
            Expr::Div(a, b) => write!(f, "({} / {})", a, b),
        }
    }
}

// ─── REFLECTION ───────────────────────────────────────────────────────────────
//
// 1. What makes Rust enums more powerful than enums in most other languages?
// I don't have experience with Enums in many other languages, but a few things that stand out to me about them in Rust,
// they can be exhaustively pattern matched against, the compiler will help to ensure this
// they are typed and get all the compiler typing info and help
// thier enumerations can hold values
// they are similar to structs in that they can have functionality and traits implemented on them
//
// 2. What does "exhaustive pattern matching" mean, and why does it matter?
// it means matching (and handling) all possible cases / states of a value
// it matters because it requires that all possible cases of a value are handled when it is matched against, no case can be missed, enforced by the compiler
//
// 3. In section 5, why does Expr need Box<Expr> rather than just Expr?
// the Expr enum, if it could contain itself directly, would be an unkown size
// using Box, it creates a pointer of a known size to the Expr it holds, that way an Expr that contains boxed versions of itself can have a known size because the boxes are a known size
//
// 4. What problem does the newtype pattern solve?
//    Give a concrete example where it would have caught a bug.
// they solve the problem when you need a distinct type(s) with possibly distict behaviors even though the underlying data / value for the type is an existing type
// struct First_Name(String);
// struct Last_Name(String);
//
// fn greet(first: First_Name, last: Last_Name) {
//      println!("Hello, {} {}!", first, last);
// }
//
// 5. When would you use if let instead of match?
// when you only need to handle one case of a value
//
// 6. What was the hardest part of this week?
// working through typing issues that I was running into with the Option combinator exercises, was able to get to solutions, and generally found that the simpler (or more straight forward) approach worked and I didn't have to fight with types
