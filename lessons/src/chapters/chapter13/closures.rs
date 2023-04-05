#![allow(unused)]
use std::thread;

use color_eyre::Result;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Black,
    Red,
    Blue,
    Green,
    White,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_black = 0;
        let mut num_white = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Black => num_black += 1,
                ShirtColor::White => num_white += 1,
                _ => (),
            }
        }
        if num_black > num_white {
            ShirtColor::Black
        } else {
            ShirtColor::White
        }
    }
}

pub fn main() -> Result<()> {
    let store = Inventory {
        shirts: vec![ShirtColor::Black, ShirtColor::White, ShirtColor::Black],
    };

    let user_pref1 = Some(ShirtColor::Red);

    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;

    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Immutable reference

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // Mutable reference

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // Cannot print here since list is mutably borrowed
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // Taking ownership

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    Ok(())
}
