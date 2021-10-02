pub trait Summary {
    // We just use method signatures rather than full methods because each type
    // that will implement it must have its own actual method
    fn summarize(&self) -> String;
    // Sometimes it's useful to have a default behavior for some or all of the
    // methods in a trait, so that implementations are not required for each
    // unique type.  One key thing to consider is that default behavior cannot
    // be accessed if a specific type implementation is used (an overriding
    // implementation of that same method)
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;

    // You can create default implementations that apply to all traits, and then
    // you can use that to call on a different function that will do the type-
    // specific work
    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implementing a trait on a type is similar to implementing regular methods
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// We need the pub because we use it in bin
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// We use the impl keyword and trait name rather than a concrete type.  This
// allows us to use this method on all types that implement that trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// This is the proper implementation of notify (full type bounds, rather than
// using the syntax sugar).  These two methods are identical
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// This can be easier to read, and maintains flexibility of types within the
// trait
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!(
        "Breaking news! {}, {}",
        item1.summarize(),
        item2.summarize()
    );
}

// But this is beeter for explicitness and forces only one type within the trait
pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!(
        "Breaking news! {}, {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify5(item: &(impl Summary + std::fmt::Display)) {
    println!("{}", item);
}

pub fn notify6<T: Summary + std::fmt::Display>(item: &T) {
    println!("{}", item);
}

// You can also use a where clause to make this easier to read as it gets more
// complicated
pub fn some_function<T: std::fmt::Display + Clone, U: Clone + std::fmt::Debug>(
    t: &T,
    u: &U,
) -> i32 {
    let out: i32 = 5;
    out
}

pub fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    let out: i32 = 5;
    out
}

// This shows that you can also return a value of some type that implements a
// trait.  You can't return more than a single trait.  This means that even
// though the types NewsArticle and Tweet are both within Summary, a method can
// only return one of them (using impl Summary, see ch 17 to do this)
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you know, people"),
        reply: false,
        retweet: false,
    }
}

use std::fmt::Display;

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // This method is always able to be implemented by Pair<T>, since there are
    // no type/ trait constraints
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // This method is only implemented if the type within Pair<T> has both the
    // Display and PartialOrd traits.
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can use to_string on anything that implements the Display trait because
// the standard library uses this blanket implementation:
// impl<T: Display> ToString for T {
//      ...
// }

// Without any lifetime annotations, rust can't tell what the lifetime of what
// this function returns to in comparison to what comes in.  It also can't
// compare the lifetimes of the two incoming arguments.
// Lifetime annotations must start with "'", typically are lowercase, and are
// short.  One lifetime annotation doesn't mean much because the purpose is to
// compare lifetimes of different variables.  Here, we're saying that x and y
// must live as long as the returned value.
// It's important to remember that we're only specifying lifetimes, not changing
// them.
// The return value needs a lifetime annotation, and any paramaters that the
// return value depends on.  It also needs to be something that has a lifetime
// that extends beyond the function, so that it can actually be transferred.
// Ultimately, lifetime annotations is all about linking the lifetimes of
// various parameters and return values of functions.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This lifetime annotation means an instance of ImportantExcerpt can't outlive
// the reference holds in its part field
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

// Rust starts by trying to apply lifetimes to everything.  The compiler has
// several patterns that it tries that are called the lifetime elision rules.
// These rules are different common cases that the compiler will consider.  If
// your function fits one of these cases, no annotations are needed.  If it does
// not fit one of the cases, lifetime annotations will be needed.  The compiler
// will not guess if there is any ambiguity.  Rather, it will throw an error and
// have you clarify.
// The Lifetime Elision Rules:
// 1- Each parameter that is a reference will get its own lifetime parameter
// 2- If there is exactly one input lifetime parameter, that lifetime is
//      assigned to all output lifetime parameters
// 3- If there are multiple input lifetime parameters, but one is &self or &mut
//      self, then the lifetime of self is assigned to all output lifetime
//      parameters
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

impl<'a> ImportantExcerpt<'a> {
    // We don't need lifetime annotations here because it doesn't return a
    // reference
    fn level(&self) -> i32 {
        3
    }

    // Here, the first rule gives each param a lifetime, and then the third rule
    // sets the output lifetime to be equal to &self
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// use std::fmt::Display;
// This is how you mix generics and lifetime annotations together
pub fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
