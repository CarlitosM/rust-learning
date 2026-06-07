// Week 2 — Borrowing in Depth
// ============================
// Prerequisites: week 1 done. You understand move vs borrow, & vs &mut.
//
// This week's theme: the borrow checker isn't arbitrary — it's enforcing a
// simple rule with far-reaching consequences. By the end you should be able
// to *predict* what the compiler will reject, and restructure code to satisfy
// it without fighting it.
//
// Run with: cargo run -p week-02-borrowing

fn main() {
    section_1_borrow_scopes();
    section_2_structs_and_borrows();
    section_3_iter_patterns();
    section_4_rc();
    section_5_putting_together();
}

// ─── SECTION 1: Borrow scopes & NLL ──────────────────────────────────────────
//
// Borrows last until their last *use*, not until the end of the block.
// This is called Non-Lexical Lifetimes (NLL) and was a big Rust improvement.
// Understanding this lets you write code that looks like it should fail but
// actually compiles fine.
//
// Task: for each example, predict whether it compiles before reading the answer.
//       Write your prediction as a comment, then uncomment to verify.

fn section_1_borrow_scopes() {
    let mut s = String::from("hello");

    // --- 1a ---
    // Prediction (compile / error?):
    // compile
    {
        let r1 = &s;
        let r2 = &s;
        println!("{} {}", r1, r2);
        // r1 and r2 are last used here — borrow ends here
    }
    s.push_str(" world"); // is this valid? why?
    // yes, because s is mutable and the borrows are already dropped, and thier usage is complete
    println!("{}", s);

    // --- 1b ---
    // Prediction: error
    let mut v = vec![1, 2, 3];
    let first = &v[0]; // immutable borrow of v
    // v.push(4); // uncomment — what error do you get and why?
    // cannot borrow `v` as mutable because it is also borrowed as immutable
    // mutable borrow occurs here
    println!("{}", first); // first used here, borrow ends here
    v.push(4); // is this valid now? why?
    // yes, because of NLL, the immutable borrow is no longer being used and v is now allowed to be mutable
    println!("{:?}", v);

    // --- 1c: the tricky one ---
    // Prediction:
    let mut scores = vec![10, 20, 30];
    let high = scores.iter().max().unwrap(); // borrows scores
    // scores.push(40); // uncomment to see the error, then re-comment
    println!("high score: {}", high);
    scores.push(40); // valid here?
    // I think so, because of NLL, the immutable borrow is no longer being used and scores is now allowed to be mutable
    println!("{:?}", scores);

    // Q: what rule explains all three examples above?
    // Your answer: NLL? I feel though that it's the borrowing rules with NLL that explain the examples above
}

// ─── SECTION 2: Structs that hold references ─────────────────────────────────
//
// When a struct holds a &str or any reference, the compiler needs to know
// how long that reference lives. This is your first real encounter with
// lifetime annotations — not as abstract syntax, but as a practical necessity.
//
// Task: make the code below compile by adding a lifetime annotation to
//       the struct. Then answer the questions.

fn section_2_structs_and_borrows() {
    // This struct holds a reference to a string rather than owning it.
    // It avoids allocation — useful when the data already exists elsewhere.
    //
    // Add a lifetime parameter to make this compile:
    struct Config<'a> {
        host: &'a str,
        path: &'a str,
    }

    // Don't change anything below this line — just make the struct work.
    let host = String::from("example.com");
    let path = String::from("/api/v1");

    let config = {
        Config {
            host: &host,
            path: &path, // path is owned by the inner block...
        }
        // ...but path is dropped here. Does the compiler catch this?
        // What error do you get?
        // `path` does not live long enough
        // borrowed value does not live long enough
    };
    // Uncomment to see if config is usable here:
    println!("{}{}", config.host, config.path);

    // --- Fix it ---
    // Move path to the same scope as host so the struct is valid.
    // Rewrite the block above so it compiles and prints host + path.

    // Q1: what does the 'a in Config<'a> mean in plain English?
    // Your answer: everything annotated with this annotation ('a) will be available while everything else annotated with the same annotation is available

    // Q2: why does Rust need you to be explicit here, when it could
    //     theoretically figure it out itself?
    // Your answer: not sure, I think it is that the compiler can't actually garauntee that the references will live long enough without the annotation
}

// ─── SECTION 3: Iteration patterns ───────────────────────────────────────────
//
// Iterating over a collection while borrowing is something you'll do constantly.
// Rust gives you three flavours — understanding the difference is essential.
//
// Task: implement all three versions of the functions below.
//       No cloning allowed unless explicitly noted.

fn section_3_iter_patterns() {
    let words = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];

    // --- 3a: iterate by immutable reference ---
    // words is still usable after this call
    print_lengths(&words);
    println!("still have words: {:?}", words);

    // --- 3b: iterate consuming the collection ---
    // words is moved into this function and dropped inside it
    let lengths = into_lengths(words);
    // words is no longer usable here
    println!("lengths: {:?}", lengths);

    // --- 3c: iterate mutably ---
    let mut numbers = vec![1, 2, 3, 4, 5];
    double_in_place(&mut numbers);
    println!("doubled: {:?}", numbers);

    // Q: which of .iter(), .into_iter(), .iter_mut() corresponds to each
    //    function above? Why?
    // Your answer:
    // print_lengths - .iter(), because that method doesn't take ownership
    // into_lengths - .into_iter(), because it takes ownership
    // double_in_place - .iter_mut(), because it creates a mutable borrow
}

fn print_lengths(words: &[String]) {
    // TODO: print each word and its length
    // Use .iter() — yields &String
    // Hint: format is  "apple: 5"
    words
        .iter()
        .for_each(|word| println!("{}: {}", word, word.len()));
}

fn into_lengths(words: Vec<String>) -> Vec<usize> {
    // TODO: consume words and return a Vec of lengths
    // Use .into_iter() — yields String (owned)
    // Hint: .map() and .collect()
    words.into_iter().map(|word| word.len()).collect()
}

fn double_in_place(numbers: &mut Vec<i32>) {
    // TODO: double every number in place
    // Use .iter_mut() — yields &mut i32
    // Hint: for n in numbers.iter_mut() { *n *= 2; }
    //       the * dereferences the mutable reference to get the value
    numbers.iter_mut().for_each(|number| *number *= 2);
}

// ─── SECTION 4: When borrowing isn't enough — Rc<T> ──────────────────────────
//
// Sometimes you genuinely need multiple owners. The borrow checker won't allow
// it with plain references. Rc<T> (reference counted) is the escape hatch for
// single-threaded shared ownership.
//
// This is NOT the default — reach for it only when you can't express ownership
// clearly. But knowing it exists stops you from hitting a wall.
//
// Task: read and run this section, then answer the questions.

fn section_4_rc() {
    use std::rc::Rc;

    // --- 4a: the problem Rc solves ---
    // Imagine two structs that both need access to the same config.
    // With plain references you'd need lifetimes threading through everything.
    // With Rc you can just clone the pointer (not the data).

    let config = Rc::new(String::from("shared-config"));

    let owner_a = Rc::clone(&config); // increments reference count
    let owner_b = Rc::clone(&config); // increments again

    println!("a: {}", owner_a);
    println!("b: {}", owner_b);
    println!("ref count: {}", Rc::strong_count(&config)); // should be 3

    drop(owner_a);
    println!("after dropping a: {}", Rc::strong_count(&config)); // should be 2

    // --- 4b: what Rc can't do ---
    // Rc doesn't allow mutation. Try uncommenting this:
    // owner_b.push_str("!"); // ERROR — why?
    // cannot mutate immutable variable `owner_b`

    // For shared mutation you'd need Rc<RefCell<T>> — we'll get there in
    // a later week. For now just note that Rc is read-only shared ownership.

    // Q1: Rc::clone doesn't deep-copy the data. What does it copy?
    // Your answer: a pointer to the data

    // Q2: why is Rc not safe to send across threads? (hint: think about
    //     what "incrementing a reference count" means without a lock)
    // Your answer: the count must be mutable, and I'm thinking, across threads, the compiler can't garuantee only one immutable borrow at a time

    // Q3: in what situation would you choose Rc over just passing &T?
    // Your answer: when using lifetime annotations would be too complex / un-maintainable
}

// ─── SECTION 5: Putting it together — a simple cache ─────────────────────────
//
// Build a struct that caches the result of an expensive computation.
// This exercises: structs with owned data, mutable methods, and borrowing
// from self in return values.
//
// Task: implement the Cache struct and its methods below.

fn section_5_putting_together() {
    let mut cache = Cache::new();

    // First call computes the value
    let v1 = cache.get_or_compute("hello");
    println!("computed: {}", v1);

    // Second call returns cached result (no recomputation)
    let v2 = cache.get_or_compute("hello");
    println!("cached: {}", v2);

    // Different key computes again
    let v3 = cache.get_or_compute("world");
    println!("computed: {}", v3);

    println!("cache size: {}", cache.size());
}

use std::collections::HashMap;

struct Cache {
    // TODO: add a field to store key -> value mappings
    // Hint: HashMap<String, String>
    store: HashMap<String, String>,
}

impl Cache {
    fn new() -> Self {
        // TODO: return a Cache with an empty HashMap
        Self {
            store: HashMap::new(),
        }
    }

    fn get_or_compute(&mut self, key: &str) -> &str {
        // TODO: if key exists in store, return the cached value
        //       if not, compute a new value, store it, and return it
        //
        // The "computation" can just be: format!("[computed: {}]", key)
        //
        // This is trickier than it looks — the borrow checker will push back.
        // Hint: use the .entry() API:
        //   self.store.entry(key.to_string()).or_insert_with(|| ...)
        //
        // Q: why does get_or_compute take &mut self?
        // I believe, because it's store might be mutated (added to) in this function
        // Q: why does it return &str instead of String?
        // becuase it needs to maintain ownership of the data, so it returns a borrowed type
        self.store
            .entry(key.to_string())
            .or_insert_with(|| format!("[computed: {}]", key))
    }

    fn size(&self) -> usize {
        // TODO: return the number of cached entries
        self.store.len()
    }
}

// ─── REFLECTION ───────────────────────────────────────────────────────────────
//
// 1. What does NLL (Non-Lexical Lifetimes) mean in practice?
// I belive that in practice, it means that you'll have to create / manage fewer lifetime annotations
// 2. When a struct holds a reference, what does the lifetime annotation
//    on the struct actually guarantee?
// that the underlying data associated with that reference will be availble (not dropped) as long as the struct is around
// 3. What's the difference between .iter(), .into_iter(), .iter_mut()?
//    Write it in one sentence each.
// .iter() creates an immutable borrow, doesn't take ownership
// .into_iter() takes ownership
// .iter_mut() creates a mutable borrow, doesn't take ownership
// 4. When would you reach for Rc<T>?
// when lifetime annotations would be too complex / un-maintainble to use
// 5. What was the hardest part of this week?
// trying to maintain a mental model for how the borrowing rules work in with or in the context of lifetimes
