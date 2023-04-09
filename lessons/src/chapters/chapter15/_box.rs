#![allow(unused)]

use color_eyre::Result;

#[derive(Debug)]
enum List {
    Cons(i32, Box<Self>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn new(value: i32) -> Self {
        Cons(value, Box::new(Nil))
    }

    fn push(&mut self, value: i32) -> &mut Self {
        match self {
            Nil => {
                *self = Self::new(value);
                self
            }
            Cons(_, cons) => Self::push(cons, value),
        }
    }
}

pub fn main() -> Result<()> {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    dbg!(list);

    let mut list = Box::new(Nil);

    list.push(1).push(2).push(3);

    dbg!(list);

    Ok(())
}
