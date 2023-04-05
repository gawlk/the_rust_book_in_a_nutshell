#![allow(unused)]

use color_eyre::Result;

mod closures;
mod iterators;

pub fn main() -> Result<()> {
    // closures::main();

    iterators::main();

    Ok(())
}
