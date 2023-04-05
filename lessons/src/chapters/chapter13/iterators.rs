#![allow(unused)]

use color_eyre::Result;

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: &[Shoe], shoe_size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(|s| s.size == shoe_size).collect()
}

pub fn main() -> Result<()> {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter.clone() {
        println!("Got: {}", val);
    }

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    dbg!(total, 6);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    dbg!(v2);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(&shoes, 10);

    dbg!(in_my_size);

    Ok(())
}
