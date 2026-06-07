// Week 4 — Generics
// =================
// Prerequisites: weeks 1–3. You understand traits, borrowing, and lifetimes.
//
// This week's theme: generics let you write one piece of code that works for
// many types — with zero runtime cost. The trait bounds you learned last week
// are the grammar of generics: they say "this works for any T, as long as T
// can do X".
//
// The JS comparison: generics are like TypeScript generics, but the bounds are
// enforced at compile time and the compiler generates specialised, fully
// optimised code for each concrete type you use. No boxing, no vtable.
//
// Run with: cargo run -p week-04-generics
use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::{self, Display, Formatter},
};

fn main() {
    section_1_generic_functions();
    section_2_generic_structs();
    section_3_multiple_bounds();
    section_4_where_clauses();
    section_5_putting_together();
}

// ─── SECTION 1: Generic functions ────────────────────────────────────────────
//
// A generic function works for any type T that satisfies the bounds.
// The compiler generates a separate, fully optimised version for each
// concrete type used — this is called monomorphisation.
//
// Task: implement the generic functions below.

fn section_1_generic_functions() {
    // --- 1a: unconstrained generic (T can be anything) ---
    // This compiles but you can't do much with T — no bounds means no methods
    fn first<T>(list: &[T]) -> Option<&T> {
        // TODO: return the first element of the slice, or None if empty
        // Hint: list.first() exists, but implement it with indexing for practice
        match list.len() {
            0 => None,
            _ => Some(&list[0]),
        }
    }

    println!("{:?}", first(&[1, 2, 3]));
    println!("{:?}", first(&["a", "b"]));
    println!("{:?}", first::<i32>(&[]));

    // --- 1b: constrained generic ---
    // PartialOrd lets us compare, Display lets us print
    fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
        // TODO: return a reference to the largest element
        // Hint: start with list.first()?, then iterate
        list.first()?;
        list.iter().max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    println!("{:?}", largest(&[3, 1, 4, 1, 5, 9, 2, 6]));
    println!("{:?}", largest(&["banana", "apple", "cherry"]));

    // --- 1c: returning owned vs borrowed ---
    // Sometimes you need to return an owned T, not a reference.
    // That requires Clone (or Copy).
    fn largest_owned<T: PartialOrd + Clone>(list: &[T]) -> Option<T> {
        // TODO: return a clone of the largest element
        let biggest = largest(list)?;
        Some(biggest.clone())
    }

    let result = largest_owned(&[3, 1, 4, 1, 5, 9]);
    println!("largest owned: {:?}", result);

    // Q: why does largest return Option<&T> but largest_owned returns Option<T>?
    // What's the tradeoff?
    // Your answer:
    // &T indicates that T is borrowed, a reference to T
    // v. just T which indicates that it is not borrowed

    // Q: why does largest_owned need the Clone bound but largest doesn't?
    // Your answer:
    // bcause the clone method is part of the Clone trait's contract
}

// ─── SECTION 2: Generic structs ───────────────────────────────────────────────
//
// Structs can be generic too. This lets you build data structures that work
// with any type — like Vec<T> or HashMap<K, V> in the standard library.
//
// Task: implement the generic structs and their methods below.

fn section_2_generic_structs() {
    // --- 2a: a simple generic wrapper ---
    let int_pair = Pair::new(1, 2);
    let str_pair = Pair::new("hello", "world");

    println!("int pair: {:?}", int_pair); // needs Debug
    println!("str pair: {:?}", str_pair);

    // This method only exists when T implements PartialOrd + Display:
    int_pair.print_largest();
    str_pair.print_largest();

    // --- 2b: a generic stack ---
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("top: {:?}", stack.peek());
    println!("pop: {:?}", stack.pop());
    println!("top after pop: {:?}", stack.peek());
    println!("size: {}", stack.size());
    println!("empty: {}", stack.is_empty());

    // Works with any type:
    let mut str_stack: Stack<String> = Stack::new();
    str_stack.push(String::from("first"));
    str_stack.push(String::from("second"));
    println!("str top: {:?}", str_stack.peek());
}

// TODO: implement Pair<T>
// - holds two values of the same type T
// - new(first: T, second: T) -> Self
// - print_largest(&self) where T: PartialOrd + Display
//   prints whichever of first/second is larger
// - derive Debug
#[derive(Debug)]
struct Pair<T> {
    // TODO
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        // TODO
        Self { first, second }
    }
}

impl<T: PartialOrd> Pair<T> {
    fn largest(&self) -> Option<&T> {
        let order = self.first.partial_cmp(&self.second)?;
        match order {
            Ordering::Equal => None,
            Ordering::Greater => Some(&self.first),
            Ordering::Less => Some(&self.second),
        }
    }
}

impl<T: PartialOrd + Display> Pair<T> {
    fn print_largest(&self) {
        // TODO: print the larger of the two values
        // format: "largest: 2"
        println!("largest: {}", self.largest().get_or_insert(&self.first));
    }
}

// TODO: implement Stack<T>
// - backed by a Vec<T>
// - push(value: T)
// - pop() -> Option<T>       (removes and returns top)
// - peek() -> Option<&T>     (returns reference to top without removing)
// - size() -> usize
// - is_empty() -> bool
struct Stack<T> {
    // TODO
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
    fn push(&mut self, value: T) {
        self.items.push(value);
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    fn size(&self) -> usize {
        self.items.len()
    }
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// ─── SECTION 3: Multiple bounds & impl Trait ─────────────────────────────────
//
// You can require multiple traits with +. The `impl Trait` syntax is
// shorthand for a generic with a bound — cleaner for simple cases.
//
// impl Trait in argument position:   fn foo(x: &impl Display)
//   = fn foo<T: Display>(x: &T)       (identical, just shorter)
//
// impl Trait in return position:     fn make_thing() -> impl Display
//   = "I return something that implements Display, but I won't say what"
//   This is different from a generic — the caller can't choose the type.

fn section_3_multiple_bounds() {
    // --- 3a: impl Trait in argument position ---
    fn print_twice(item: &impl Display) {
        println!("{} {}", item, item);
    }
    print_twice(&42);
    print_twice(&"hello");

    // --- 3b: impl Trait in return position ---
    fn make_greeting(name: &str) -> impl Display {
        // TODO: return something that implements Display
        // (a String is fine — the caller just sees "something printable")
        format!("hello {}", name)
    }
    println!("{}", make_greeting("world"));

    // --- 3c: YOUR TURN ---
    // Implement summarise() below using impl Trait syntax (not explicit generics)
    // It takes anything that is Display + Clone, clones it, and prints both
    // the original and clone with a label.
    // format: "original: hello, clone: hello"
    fn summarise(item: &(impl Display + Clone)) {
        // TODO
        println!("original: {}, clone: {}", item, item.clone())
    }
    summarise(&String::from("hello"));
    summarise(&99i32);

    // Q: when would you prefer `fn foo<T: Display>(x: &T)` over
    //    `fn foo(x: &impl Display)`? Are they always interchangeable?
    // Your answer:
}

// ─── SECTION 4: Where clauses ────────────────────────────────────────────────
//
// When bounds get complex, where clauses keep signatures readable.
// They're purely stylistic — same semantics as inline bounds.
//
// Task: rewrite the functions using where clauses, then implement them.

fn section_4_where_clauses() {
    // --- 4a: rewrite this with a where clause ---
    // Original:  fn compare_and_print<T: PartialOrd + Display>(a: T, b: T)
    // Your version:
    fn compare_and_print<T>(a: T, b: T)
    where
        T: PartialOrd + Display,
    {
        // TODO: print which is larger, or "equal" if they're the same
        // format: "3 > 2" or "2 < 3" or "3 == 3"
        let comparitor = match a.partial_cmp(&b).get_or_insert(Ordering::Equal) {
            Ordering::Equal => "==",
            Ordering::Greater => ">",
            Ordering::Less => "<",
        };

        println!("{} {} {}", a, comparitor, b);
    }
    compare_and_print(3, 2);
    compare_and_print("apple", "banana");
    compare_and_print(42, 42);

    // --- 4b: multiple type parameters ---
    // This has two type parameters with different bounds — where clause
    // keeps it clean.
    fn zip_display<T, U>(items_t: &[T], items_u: &[U])
    where
        T: Display,
        U: Display,
    {
        // TODO: print each pair side by side, stopping at the shorter slice
        // format: "1 - apple", "2 - banana", etc.
        items_t
            .iter()
            .zip(items_u)
            .for_each(|(a, b)| println!("{} - {}", a, b));
    }
    zip_display(&[1, 2, 3], &["apple", "banana", "cherry"]);
    zip_display(&["x", "y"], &[10, 20, 30]); // stops after 2 pairs
}

// ─── SECTION 5: Putting it together — a generic event queue ──────────────────
//
// Build a generic EventQueue<T> that:
//   - stores events of any type T
//   - lets you push events and process them in order (FIFO)
//   - can peek at the next event without consuming it
//   - tracks how many events have been processed total
//   - implements Display when T: Display (show queue contents)
//   - has a drain() method that processes all remaining events
//     using a provided closure
//
// This exercises: generic structs, trait bounds on impl blocks,
// closures as arguments, and iterators.

fn section_5_putting_together() {
    // Integer events
    let mut queue: EventQueue<i32> = EventQueue::new();
    queue.push(10);
    queue.push(20);
    queue.push(30);

    println!("next: {:?}", queue.peek());
    println!(
        "processed: {}",
        queue.process_next(|e| println!("handling: {}", e))
    );
    println!("total processed: {}", queue.total_processed());
    println!("remaining: {}", queue.len());

    // String events
    let mut str_queue: EventQueue<String> = EventQueue::new();
    str_queue.push(String::from("login"));
    str_queue.push(String::from("purchase"));
    str_queue.push(String::from("logout"));

    println!("\n{}", str_queue); // Display

    str_queue.drain(|e| println!("processing event: {}", e));
    println!(
        "after drain — total processed: {}",
        str_queue.total_processed()
    );
    println!("after drain — remaining: {}", str_queue.len());
}

struct EventQueue<T> {
    // TODO: fields
    // Hint: VecDeque<T> is an efficient double-ended queue
    //       (use it instead of Vec for FIFO)
    queue: VecDeque<T>,
    processed: usize,
}

impl<T> EventQueue<T> {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            processed: 0,
        }
    }

    fn push(&mut self, event: T) {
        self.queue.push_back(event);
    }

    fn peek(&self) -> Option<&T> {
        self.queue.front()
    }

    // Process the next event using the provided closure.
    // Returns true if an event was processed, false if queue was empty.
    fn process_next(&mut self, handler: impl FnOnce(T)) -> bool {
        match self.queue.pop_back() {
            None => false,
            Some(event) => {
                handler(event);
                self.processed += 1;
                true
            }
        }
    }

    // Process all remaining events using the provided closure.
    fn drain(&mut self, handler: impl Fn(T)) {
        self.queue.drain(..).for_each(|event| {
            handler(event);
            self.processed += 1;
        });
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn total_processed(&self) -> usize {
        self.processed
    }
}

impl<T: Display> Display for EventQueue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: print all events in the queue
        // format: "EventQueue[login, purchase, logout]"
        write!(
            f,
            "EventQueue[{}]",
            self.queue
                .iter()
                .map(|event| format!("{}", event))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

// ─── REFLECTION ───────────────────────────────────────────────────────────────
//
// 1. What is monomorphisation? Why does it matter for performance?
//    it is a one to one (mono) generation of a concreate function definition for each type actually used
//    it matters for perfomance because this does not incure any runtime overhead as it would if it was otherwise dynamic at runtime
//
// 2. What's the difference between these two signatures:
//      fn foo<T: Display>(x: T)
//      fn foo(x: &dyn Display)
//    When would you choose each?
//    fn foo<T: Display>(x: T) - gets monomorphised into concrete definitions
//    fn foo(x: &dyn Display) - dynamically dispatched, type determined at runtime
//    not entirely sure when you would need to choose the &dyn option, seems like it would be required if there is no way of determing the concrete type at compile time
//
// 3. When does a generic function need a Clone bound?
//    when it needs to clone/copy one of it's inputs, seems useful mostly for being able to return an owned value
//
// 4. What does `impl Trait` in return position give you that
//    generics can't? What does it take away?
//    it allows the function to determine the return type
//    it means that the caller of the function can't determine the return type
//
// 5. What was the hardest part of this week?
//    one of the things that I have really enjoyed about javascript is it's loose type system and that it has the tools to check types when I need to
//    I feel like strong typing incurs more code (sometimes much more code), can waterfall through your code (similar to how async syntax can do this) and can be very difficult to resolve typing errors
//    this is why I never adopted Typescript and stuck with javascript
//    that being said, I do appreciate what strong typing gives you and feel that Rust's type system is one of the best
