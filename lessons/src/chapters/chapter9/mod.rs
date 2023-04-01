#![allow(unused)]
use color_eyre::Result;

mod panic;
mod result;

pub fn main() -> Result<()> {
    panic::main();

    result::main()?;

    Ok(())
}
