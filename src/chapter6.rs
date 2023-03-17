#![allow(dead_code, unused_variables)]
use color_eyre::Result;

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

enum IpAddr {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

// It's possible to group the following structs
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Through an enum like so
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    #[allow(clippy::manual_map)]
    // Shoudl be: x.map(|x| x + 1)
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn main() -> Result<()> {
    // Enum creation

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 0);

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // Match

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(five, six, none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    let opt: Option<String> = Some(String::from("Hello world"));

    // opt became &opt without the reference it won't compile
    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);

    // If let
    // Sugar coating for running some code for only one branch of a match and discarding the others
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // Possibility 1
    let coin = Coin::Penny;
    let mut _count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => _count += 1,
    }

    // Possibility 2
    let coin = Coin::Penny;
    let mut _count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        _count += 1;
    }

    Ok(())
}
