#![allow(dead_code, unused)]

use color_eyre::Result;

pub fn main() -> Result<()> {
    let mut s = String::new();

    // Equivalent
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = "Hello".to_string();
    s.push_str(" Bob !");
    dbg!(s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Calls .add(), the first parameter is eaten and the second parameter has to be a reference

    // Which is why format! is preffered
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    dbg!(s);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    dbg!(s); // Зд and not Здра !!
    dbg!(hello.chars()); // Fixes the issue

    dbg!("नमस्ते".chars()); // But even chars are not always valid, should've been 4 letters, but quite hard so not possible natively

    dbg!("😀🥸".chars());

    Ok(())
}
