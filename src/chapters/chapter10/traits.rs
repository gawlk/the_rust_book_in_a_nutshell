#![allow(unused)]
use std::fmt::{Debug, Display};

use color_eyre::Result;

trait Summary {
    fn print_sum(&self) {
        println!("sum");
    }

    fn summarize_author(&self) -> String;

    // With default behavior
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Non sugar coated version
    fn notify2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // Argument having multiple trait
    fn notify3(item: &(impl Summary + Display)) {
        println!("Breaking news! {}", item.summarize());
    }

    // Non sugar coated version bis
    fn notify4<T: Summary + Display>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // Can end up quite long
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        1
    }

    // In which case we can use `where` like so
    fn some_function2<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        1
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Implement cmp_display only if T implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

trait Bob {
    fn print_bob() {
        println!("booooob");
    }
}

// Implement Bob for all types
impl<T> Bob for T {
    // --snip--
    // fn a() {
    //     println!("a");
    // }
}

trait Billy {
    // Default print
    fn print_billy(&self) {
        println!("plain ol' billy")
    }
}

// Implement Billy for all types implementing Summary
impl<T: Summary> Billy for T {
    // --snip--
    fn print_billy(&self) {
        println!("summary billy !");
    }
}

pub fn main() -> Result<()> {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // Notify doesn't has the self parameter so have to call it like so
    Tweet::notify(&tweet);

    // Same for Bob
    Tweet::print_bob();

    // But with &self we can do
    tweet.print_billy();

    Ok(())
}
