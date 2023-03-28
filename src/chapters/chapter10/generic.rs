#![allow(unused)]
use color_eyre::Result;

fn max_i32(list: &[i32]) -> Option<&i32> {
    list.iter().max()
}

fn max_char(list: &[char]) -> Option<&char> {
    list.iter().max()
}

fn max<T: std::cmp::Ord>(list: &[T]) -> Option<&T> {
    list.iter().max()
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Only for Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct AnarchistPoint<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> AnarchistPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: AnarchistPoint<X2, Y2>) -> AnarchistPoint<X1, Y2> {
        AnarchistPoint {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn main() -> Result<()> {
    let number_list = vec![34, 50, 25, 100, 65];

    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    println!("The largest number is {:#?}", max(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is {:#?}", max(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("The largest char is {:#?}", max(&char_list));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let integer_and_float = AnarchistPoint { x: 5, y: 4.0 };

    dbg!(float);

    Ok(())
}
