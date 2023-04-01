#![allow(unused)]
use color_eyre::Result;

mod map;
mod string;
mod vec;

pub fn main() -> Result<()> {
    vec::main()?;
    string::main()?;
    map::main()?;

    Ok(())
}
