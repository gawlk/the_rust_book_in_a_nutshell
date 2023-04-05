//! # My Crate (Lessons)
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

#![allow(unused)]
// Point of entry for library crates
// mod chapters;
mod chapters;

use crate::chapters::chapter7::Appéritif;
pub use chapters::chapter14::*; // pub all from chapter14
use chapters::chapter5::Rectangle;
use chapters::chapter7;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

pub fn add_two2(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)] // To be ran/compiled only during tests
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_two_fail() {
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);
        assert!(
            result.contains(name),
            "Greeting did not contain name `{}`, value was `{}`",
            name,
            result
        );
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    #[should_panic(expected = "must be less")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "must be greater")]
    fn less_than_1() {
        Guess::new(0);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_doesnt_work() {
        assert!(Err::<(), String>(String::from("two plus two does not equal four")).is_err());
    }

    fn prints_and_returns_10(a: i32) -> i32 {
        println!("I got the value {}", a);
        10
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(1, 1);
    }
}
