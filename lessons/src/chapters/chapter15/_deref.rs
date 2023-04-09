#![allow(unused, clippy::deref_addrof)]

use color_eyre::Result;
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn main() -> Result<()> {
    let x = 5;

    dbg!(5 == *&x);

    dbg!(5 == *Box::new(x));

    dbg!(5 == *MyBox::new(x));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    Ok(())
}
