#![allow(unused)]
use color_eyre::Result;
use std::fmt::Display;

mod generic;
mod lifetimes;
mod traits;

pub fn main() -> Result<()> {
    // generic::main();

    // traits::main();

    lifetimes::main();

    Ok(())
}

// Recap
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
