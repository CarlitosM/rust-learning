fn main() {
    section_1_moves();
    section_2_clone();
    section_3_borrowing();
    section_4_mutable_refs();
    section_5_slices();
    stretch_goal();
}

// Week 1 — Ownership & Move Semantics
// =====================================
// Work through each section in order.
// Uncomment the next block only after the current one compiles and you
// understand *why* it compiles (or why the commented-out lines don't).
//
// Run with:  cargo run -p week-01-ownership

// ─── SECTION 1: Move semantics ───────────────────────────────────────────────
//
// In JS:  let b = a  always copies (primitives) or shares a reference (objects).
// In Rust: let b = a  MOVES ownership for heap types (String, Vec, etc.).
//          After a move, the original binding is gone.
//
// Task: read the code, then answer in a comment below each block:
//       "Why does this compile / not compile?"

fn section_1_moves() {
    // --- 1a: this compiles fine ---
    let s1 = String::from("hello");
    let s2 = s1; // s1 is *moved* into s2
    println!("{}", s2);
    // Q: why can't you println!("{}", s1) here?
    // Your answer: because s1 was moved into s2 and effectively dropped, I think

    // --- 1b: integers are different — they implement Copy ---
    let x = 5;
    let y = x; // x is *copied*, not moved
    println!("{} {}", x, y); // both still valid
    // Q: why does this work when String didn't?
    // Your answer: because x's value was copied and not moved, so both x and y should be pointing to a value in memory at this point

    // --- 1c: YOUR TURN ---
    // Uncomment these lines one at a time and predict the compiler error
    // before you see it. Then fix each one without using clone().
    //
    let v1 = vec![1, 2, 3];
    let v2 = &v1; // this changes the type of v2 though so that it's not quite the same type that v1 is, right?
    println!("{:?}", v1); // fix: how do you print v1 without cloning?
    //
    // Hint: you need to borrow v1 when creating v2, not move it.
    // Think about what & does.
}

// ─── SECTION 2: Clone — the escape hatch ─────────────────────────────────────
//
// .clone() explicitly deep-copies heap data. It's always valid but has a cost.
// Knowing *when* to clone vs. borrow is a core Rust skill.
//
// Task: fix the function below so it compiles, using clone() exactly once.
//       Then rewrite it a second time using borrowing instead of clone().
//       Which version would you prefer in real code and why?

fn section_2_clone() {
    let original = String::from("ownership");

    // This function takes ownership of its argument:
    fn takes_ownership(s: &String) {
        println!("got: {}", s);
    }

    takes_ownership(&original); // clone so original survives
    println!("still have: {}", original);

    // --- YOUR TURN ---
    // Rewrite takes_ownership so it borrows instead of taking ownership.
    // You'll need to change both the function signature and the call site.
    // What does the & in the signature mean?
}

// ─── SECTION 3: Immutable borrowing ──────────────────────────────────────────
//
// A borrow (&T) lets you use a value without taking ownership.
// Multiple immutable borrows can coexist — Rust guarantees no one is mutating
// the data while you're reading it.
//
// Task: implement the two functions below. Don't use .clone() anywhere here.

fn section_3_borrowing() {
    let sentence = String::from("the quick brown fox");

    // Call your implementations:
    let wc = word_count(&sentence);
    let first = first_word(&sentence);
    println!("words: {}, first: {}", wc, first);

    // sentence is still valid here — borrows don't consume it
    println!("original: {}", sentence);
}

fn word_count(s: &str) -> usize {
    // TODO: count the words in s (split on whitespace)
    // Hint: s.split_whitespace() returns an iterator
    s.split_whitespace().count()
}

fn first_word(s: &str) -> &str {
    // TODO: return the first word (the part before the first space)
    // Hint: s.find(' ') returns Option<usize>
    //       &s[..index] gives you a string slice up to that index
    // Important: notice the return type is &str, not String.
    //            You're returning a *slice* of the input — no allocation needed.
    s.split_whitespace().nth(0).unwrap()
}

// ─── SECTION 4: Mutable references ───────────────────────────────────────────
//
// Rust's rule: you can have EITHER
//   - any number of immutable borrows (&T), OR
//   - exactly ONE mutable borrow (&mut T)
// Never both at the same time.
//
// This prevents data races at compile time — a guarantee JS can never make.
//
// Task: implement append_exclamation below.
//       Then try uncommenting the broken examples and read the compiler errors.

fn section_4_mutable_refs() {
    let mut s = String::from("hello");
    append_exclamation(&mut s);
    println!("{}", s); // should print "hello!"

    // --- broken examples (uncomment to see the errors) ---

    // Example A: two mutable borrows at once
    // let r1 = &mut s;
    // let r2 = &mut s;  // ERROR — can't borrow s as mutable more than once
    // println!("{} {}", r1, r2);

    // Example B: mutable + immutable borrow at the same time
    // let r1 = &s;
    // let r2 = &mut s;  // ERROR — can't borrow as mutable while immutable borrow exists
    // println!("{} {}", r1, r2);

    // Q: why does Rust care about these cases?
    // Think about: what could go wrong in a multi-threaded program if both were allowed?
    // Your answer: in regards to 2 mutable references, that could create a data race scenario where the underlying value is no longer deterministic
    // in regards to an immutable borrow with a mutable borrow, at it's core those are mutally exclusive, the underlying value can't both be mutable and immutable at the same time
}

fn append_exclamation(s: &mut String) {
    // TODO: push '!' onto s
    // Hint: s.push('!')  or  s.push_str("!")
    s.push('!');
}

// ─── SECTION 5: String slices — putting it together ──────────────────────────
//
// &str is a *slice* — a reference to a portion of string data.
// It's the borrowed view of either a String or a string literal.
// Prefer &str in function signatures over &String — it's more flexible.
//
// Task: implement the function below, then answer the questions in comments.

fn section_5_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // slice of s
    let world = &s[6..11]; // another slice of s

    println!("{} {}", hello, world);

    // Your function:
    let upper_count = count_uppercase(&s);
    println!("uppercase letters: {}", upper_count);

    // Q1: &s[0..5] doesn't allocate. Where does hello point?
    // Your answer: points to the underlying value of s, in the given range

    // Q2: what would happen if you tried to drop s while hello is still in scope?
    // Your answer: compiler error :) the unerlying value that hello is pointing to would no longer be garuanteed to be the value that was originally allocated at that address

    // --- STRETCH GOAL ---
    // Implement a function:  fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str
    // that returns whichever string is longer.
    // Don't worry if the lifetime syntax ('a) looks weird — we'll cover it properly
    // in week 3. Just try to make it compile and notice what the compiler asks for.
}

fn count_uppercase(s: &str) -> usize {
    // TODO: count characters in s that are uppercase
    // Hint: s.chars() gives an iterator of char
    //       char has an .is_uppercase() method
    s.chars().filter(|c| c.is_uppercase()).count()
}

fn stretch_goal() {
    let s = longest("the", "longest");
    println!("the longest is {}", s);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// ─── REFLECTION (fill in before moving to week 2) ────────────────────────────
//
// 1. What's the difference between moving and borrowing?
// moving moves the underlying value in memory, borrowing points to the underlying value in memory
// 2. When would you choose clone() over borrowing?
// if the allocation for cloning is acceptable and/or borrowing is imposible or overly complex
// 3. What does Rust's borrow rule ("one mutable OR many immutable") prevent?
// it prevents data unpredictability
// 4. What surprised you most this week?
// nothing really, was already pretty familiar with these concepts, I need practice and more experience with them before I feel confident with them though
