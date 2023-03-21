#![allow(dead_code, unused)]

use std::collections::HashMap;

use color_eyre::Result;

pub fn main() -> Result<()> {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let mut scores = dbg!(scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    dbg!(score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Takes ownership of both strings

    // Overwrite
    scores.insert(String::from("Blue"), 25);
    // Set if not set
    scores.entry(String::from("Blue")).or_insert(50);

    // Example: Word counter
    let text = "hello    world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    fn reverse(v: &mut Vec<String>) {
        let n = v.len();
        for i in 0..n / 2 {
            let p1 = &mut v[i] as *mut String;
            let p2 = &mut v[n - i - 1] as *mut String;
            unsafe {
                std::ptr::swap_nonoverlapping(p1, p2, 1);
            }
        }
    }

    Ok(())
}
