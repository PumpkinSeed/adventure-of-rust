// NOTE TO generics: use traits in generic
use std::fmt;
fn p<T: fmt::Debug>(x: T) { // all type which implements the Debug trait
    println!(":?", x);
}

// traits like interfaces with some differences
// define shared behaviour in an abstract way

// type's behaviour: methods we can call on that type
pub trait Summary {
    fn summarize(&self) -> String;
}

// ------------------------------
// Implementing a Trait on a Type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {} by ({})", self.headline, self.author, self.location)
    }
}

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
}

fn print_tweet() {
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// use the trait from a library:
// use aggregator::Summary;
// restriction: we can implement a trait on a type only if either the trait or the type is local to our crate
// so we can't implement external traits on external types
// this restriction is part of a property of programs called coherence, and more specifically the orphan rule

// -----------------------
// Default Implementations

pub trait SummaryDef {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
// it has a default implementation and can be overwrite
// default ex:

impl SummaryDef for NewsArticle{}

// we can call all functions inside the trait what the trait defines

pub trait SummaryCall {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// --------------------
// Traits as Parameters

// all type implements the Summary behaviour
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax
pub fn notify_b<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

use std::fmt::Display;
// specifying multiple trait bounds
pub fn notify_m(item: impl Summary + Display) {}
pub fn notify_b_m<T: Summary + Display>(item: T) {}

// clearer trait bounds with where clauses
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {
}

// -------------------------------------
// Returning Types that implement Traits

fn returns_summarizable() -> impl Summary {
    let t = Tweet::new();
    t.username = String::from("test username");
    t
}

// -----------------------
// Fixing largest function

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// -----------------------------------------------------
// Using Trait Bounds to conditionally implement methods

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}