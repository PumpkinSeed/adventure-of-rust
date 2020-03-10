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

