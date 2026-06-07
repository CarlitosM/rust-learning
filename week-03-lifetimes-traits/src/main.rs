use std::{
    cmp,
    f64::{self, consts::PI},
    fmt,
};
// not sure why use std::f64::{consts::PI, powi, sqrt}; doesn't work for powi and sqrt
// Week 3 — Lifetimes Explicitly + Traits Intro
// ==============================================
// Prerequisites: weeks 1 & 2 done.
//
// This week's theme: lifetimes are just the compiler asking
// "how long does this reference live?" — once you can answer
// that question, the syntax becomes mechanical. Traits are how
// Rust does polymorphism — cleanly, with zero runtime cost by default.
//
// Run with: cargo run -p week-03-lifetimes-traits

fn main() {
    section_1_lifetime_elision();
    section_2_lifetime_functions();
    section_3_traits_basics();
    section_4_trait_defaults_and_bounds();
    section_5_putting_together();
}

// ─── SECTION 1: Lifetime elision — why you don't always need 'a ──────────────
//
// Rust has elision rules: in common patterns, the compiler infers lifetimes
// so you don't have to write them. Knowing the rules tells you *when* you
// DO need to be explicit.
//
// The three elision rules:
//   1. Each reference parameter gets its own lifetime.
//   2. If there's exactly one input lifetime, it's assigned to all outputs.
//   3. If one of the inputs is &self or &mut self, its lifetime is assigned
//      to all outputs.
//
// Task: for each function below, write out the *full* signature with explicit
//       lifetimes as a comment. Then explain which rule(s) apply.

fn section_1_lifetime_elision() {
    // --- 1a ---
    fn first_word<'a>(s: &'a str) -> &'a str {
        match s.find(' ') {
            Some(i) => &s[..i],
            None => s,
        }
    }
    // Full signature with explicit lifetimes:
    // fn first_word<??>(s: &?? str) -> &?? str
    // Which rule applies?
    // Your answer: 2

    // --- 1b ---
    fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() >= s2.len() { s1 } else { s2 }
    }
    // Q: why can't elision handle this one? Why must you write 'a explicitly?
    // Your answer: because it would otherwise be ambiguous as to which input lifetime the output should be tied to

    // --- 1c ---
    struct Wrapper<'a> {
        value: &'a str,
    }
    impl<'a> Wrapper<'a> {
        fn get<'b>(&'b self) -> &'b str {
            self.value
        }
    }
    // Full signature of get() with explicit lifetimes:
    // fn get<??>(& ?? self) -> & ?? str
    // Which rule applies?
    // Your answer: 3

    let w = Wrapper { value: "hello" };
    println!("{}", first_word("hello world"));
    println!("{}", longer("short", "longer one"));
    println!("{}", w.get());
}

// ─── SECTION 2: Lifetime annotations on functions ────────────────────────────
//
// When a function returns a reference, the compiler needs to know which
// input that reference comes from. That's all lifetime annotations do —
// they link outputs to inputs.
//
// Task: implement the functions below. The signatures are given —
//       your job is to make the bodies work correctly.

fn section_2_lifetime_functions() {
    // --- 2a ---
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("longest: {}", result); // must use result inside s2's scope
    }
    // Q: why can't you use result here (after the inner block)?
    // Try uncommenting: println!("{}", result);
    // Your answer: because the output would outlive one of the inputs (s2) as defined in the function signature

    // --- 2b: a function that always returns from one input ---
    let announcement = String::from("new version released");
    let sentence = String::from("Rust is great");
    let first = first_word_announced(sentence.as_str(), announcement.as_str());
    println!("first word: {}", first);
    // Q: look at the signature below — why does 'b not appear in the output?
    // Your answer: because the input that the 'b lifetime is associated with in unused by the function so it's lifetime is not related to the ouput's lifetime in any way
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// This always returns a reference into `sentence`, never `announcement`
// so only 'a needs to appear in the return type
fn first_word_announced<'a, 'b>(sentence: &'a str, _announcement: &'b str) -> &'a str {
    // TODO: print the announcement, then return the first word of sentence
    // Reuse your first_word logic from section 1
    match sentence.find(' ') {
        Some(i) => &sentence[..i],
        None => sentence,
    }
}

// ─── SECTION 3: Traits — the basics ──────────────────────────────────────────
//
// A trait defines behaviour. Any type that implements the trait provides
// that behaviour. This is Rust's equivalent of an interface — but more powerful.
//
// Task: define the trait and implement it for multiple types.

fn section_3_traits_basics() {
    let circle = Circle { radius: 3.0 };
    let rect = Rectangle {
        width: 4.0,
        height: 5.0,
    };
    let triangle = Triangle {
        base: 6.0,
        height: 4.0,
    };

    // These should all work via the trait:
    print_shape_info(&circle);
    print_shape_info(&rect);
    print_shape_info(&triangle);

    // Trait objects — a Vec of mixed shape types:
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle {
            width: 2.0,
            height: 3.0,
        }),
        Box::new(Triangle {
            base: 4.0,
            height: 5.0,
        }),
    ];
    let total_area: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("total area: {:.2}", total_area);
}

// TODO: define a Shape trait with:
//   - area(&self) -> f64
//   - perimeter(&self) -> f64
//   - name(&self) -> &str
//
// Then implement it for Circle, Rectangle, and Triangle below.
// Use std::f64::consts::PI for circle calculations.
#[derive(Debug)]
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

trait Shape {
    // TODO: declare the three methods
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &str;

    fn is_large(&self) -> bool {
        self.area() > 20.
    }

    fn largest(&self, other: &dyn Shape) -> cmp::Ordering {
        let area1 = self.area();
        let area2 = other.area();
        if area1 > area2 {
            return cmp::Ordering::Greater;
        }

        if area1 < area2 {
            return cmp::Ordering::Less;
        }

        cmp::Ordering::Equal
    }

    fn smallest(&self, other: &dyn Shape) -> cmp::Ordering {
        let area1 = self.area();
        let area2 = other.area();
        if area1 > area2 {
            return cmp::Ordering::Less;
        }

        if area1 < area2 {
            return cmp::Ordering::Greater;
        }

        cmp::Ordering::Equal
    }
}

// TODO: impl Shape for Circle
// area = PI * r^2
// perimeter = 2 * PI * r
impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * f64::powi(self.radius, 2)
    }

    fn perimeter(&self) -> f64 {
        2. * PI * self.radius
    }

    fn name(&self) -> &str {
        "Circle"
    }

    fn is_large(&self) -> bool {
        self.area() > 50.
    }
}

// TODO: impl Shape for Rectangle
// area = width * height
// perimeter = 2 * (width + height)
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        (self.width + self.height) * 2.
    }

    fn name(&self) -> &str {
        "Rectangle"
    }
}

// TODO: impl Shape for Triangle
// area = 0.5 * base * height
// perimeter: we only have base and height — assume it's isoceles.
//   the two equal sides = sqrt((base/2)^2 + height^2)
//   perimeter = base + 2 * equal_side
// name: "triangle"
impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn perimeter(&self) -> f64 {
        self.base + 2. * f64::sqrt(f64::powi(self.base / 2., 2) + f64::powi(self.height / 2., 2))
    }

    fn name(&self) -> &str {
        "Triangle"
    }
}

fn print_shape_info(shape: &dyn Shape) {
    // TODO: print name, area (2 decimal places), perimeter (2 decimal places)
    // format: "circle — area: 28.27, perimeter: 18.85"
    println!(
        "{} — area: {}, perimeter: {}",
        shape.name().to_lowercase(),
        format!("{:.2}", shape.area()),
        format!("{:.2}", shape.perimeter())
    )
}

// ─── SECTION 4: Default methods & trait bounds ────────────────────────────────
//
// Traits can provide default method implementations — types get them for free
// unless they override. Trait bounds let functions work with any type that
// implements a trait.
//
// Task: extend your Shape trait with a default method, then write generic
//       functions that work with any Shape.

fn section_4_trait_defaults_and_bounds() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 3.0,
            height: 7.0,
        }),
        Box::new(Triangle {
            base: 8.0,
            height: 3.0,
        }),
    ];

    // --- 4a: default method ---
    // Add a method  is_large(&self) -> bool  to the Shape trait
    // with a default implementation: area > 20.0
    // Then override it for Circle: area > 50.0
    for shape in &shapes {
        println!("{} is large: {}", shape.name(), shape.is_large());
    }

    // --- 4b: generic function with trait bound ---
    // Implement largest_shape below — it should work for any slice of shapes
    let largest = largest_shape(&shapes);
    println!("largest: {}", largest.name());

    // --- 4c: multiple trait bounds ---
    // Rust lets you require multiple traits: fn foo<T: TraitA + TraitB>(...)
    // Add #[derive(Debug)] to Circle and implement Display for it below.
    // Then call print_debug_and_display with a Circle.
    let c = Circle { radius: 2.5 };
    print_debug_and_display(&c);
}

// TODO: add is_large() default method to the Shape trait above (keep it up there,
//       not here) and override it for Circle

fn largest_shape(shapes: &[Box<dyn Shape>]) -> &dyn Shape {
    // TODO: return a reference to the shape with the largest area
    // Hint: use .iter().max_by() or fold
    shapes
        .iter()
        .max_by(|a, b| a.largest(b.as_ref()))
        .unwrap()
        .as_ref()
}

// TODO: implement fmt::Display for Circle
// format: "Circle(r=2.50)"
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(r={:.2})", &self.name(), &self.radius)
    }
}

fn print_debug_and_display<T: fmt::Debug + fmt::Display>(item: &T) {
    // TODO: print item using both {:?} and {}
    println!("debug: {:?}, display: {}", item, item);
}

// ─── SECTION 5: Putting it together — a shape report ─────────────────────────
//
// Use everything from this week to build a ShapeCollection that:
//   - stores shapes as trait objects
//   - can report total area, total perimeter, largest shape, smallest shape
//   - can filter by "large" shapes
//   - implements Display to print a summary
//
// This is the most open-ended exercise yet — the method signatures are given
// but the implementations are yours to figure out.

fn section_5_putting_together() {
    let mut collection = ShapeCollection::new();
    collection.add(Box::new(Circle { radius: 1.0 }));
    collection.add(Box::new(Circle { radius: 5.0 }));
    collection.add(Box::new(Rectangle {
        width: 4.0,
        height: 6.0,
    }));
    collection.add(Box::new(Triangle {
        base: 3.0,
        height: 4.0,
    }));

    println!("total area:      {:.2}", collection.total_area());
    println!("total perimeter: {:.2}", collection.total_perimeter());
    println!("largest:  {}", collection.largest().name());
    println!("smallest: {}", collection.smallest().name());

    println!("\nlarge shapes:");
    for shape in collection.large_shapes() {
        println!("  {}", shape.name());
    }

    println!("\n{}", collection); // uses your Display impl
}

struct ShapeCollection {
    shapes: Vec<Box<dyn Shape>>,
}

impl ShapeCollection {
    fn new() -> Self {
        Self { shapes: Vec::new() }
    }
    fn add(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }
    fn total_area(&self) -> f64 {
        self.shapes.iter().map(|f| f.area()).sum()
    }
    fn total_perimeter(&self) -> f64 {
        self.shapes.iter().map(|f| f.perimeter()).sum()
    }
    fn largest(&self) -> &dyn Shape {
        self.shapes
            .iter()
            .max_by(|a, b| a.largest(b.as_ref()))
            .unwrap()
            .as_ref()
    }
    fn smallest(&self) -> &dyn Shape {
        self.shapes
            .iter()
            .min_by(|a, b| a.smallest(b.as_ref()))
            .unwrap()
            .as_ref()
    }
    fn large_shapes(&self) -> Vec<&dyn Shape> {
        self.shapes
            .iter()
            .filter(|shape| shape.is_large())
            .map(|shape| shape.as_ref())
            .collect()
    }
}

impl fmt::Display for ShapeCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: print a summary, e.g.:
        // "ShapeCollection: 4 shapes, total area: 101.27"
        write!(
            f,
            "ShapesCollection: {} shapes, total area: {:.2}",
            self.shapes.iter().count(),
            self.total_area()
        )
    }
}

// ─── REFLECTION ───────────────────────────────────────────────────────────────
//
// 1. What are the three lifetime elision rules? Write them in your own words.
//   1. Each reference parameter gets its own lifetime. - there is a specific lifetime associated with every parameter passed to a function
//   2. If there's exactly one input lifetime, it's assigned to all outputs. - the lifetime of the one input to a function has to match the lifetime of all the outputs of that function
//   3. If one of the inputs is &self or &mut self, its lifetime is assigned
//      to all outputs. - the lifetime of all outputs of an instance's method(s) have to match the lifetime of the instance
// 2. When you write 'a on a function, what are you actually telling the compiler?
//    that everything in this function with that annotation will live for the same amount of time
// 3. What's the difference between a trait and a struct?
//    structs are like blueprints for concrete instances that can define both data and functionality
//    traits are abstractions that define functionality that can be shared by many structs
// 4. What's the difference between  fn foo(s: &impl Shape)
//    and  fn foo(s: &dyn Shape)?
//    (hint: one is compile-time, one is runtime — what does that mean practically?)
//    &impl (compile-time) - means that the compiler will have information about the trait when compiling and can help you, might mean slightly longer compile times, but lower memory use at runtime
//    &dyn (runtime) - provides more flexibilty when authoring the code, but less help from the compiler and more memory usage at runtime
// 5. What was the hardest part of this week?
// the compiler errors regarding type issues
