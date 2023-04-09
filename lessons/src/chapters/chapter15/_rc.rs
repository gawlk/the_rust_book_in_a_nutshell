#![allow(unused, clippy::deref_addrof)]

use color_eyre::Result;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<Self>),
    Nil,
}

use List::{Cons, Nil};

pub fn main() -> Result<()> {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));

    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    Ok(())
}
