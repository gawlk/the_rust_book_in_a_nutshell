use std::{
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

use color_eyre::Result;

pub fn main() -> Result<()> {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Or we could just do
    // File::open("hello.txt")?;
    // Thanks to color_eyre result

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// main with Result with any kind of error type
fn main2() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
