use color_eyre::Result;

mod chapter2;
mod chapter3;
mod chapter4;
mod chapter5;
mod chapter6;
pub mod chapter7;
mod chapter8;
mod chapter9;

pub fn main() -> Result<()> {
    // chapter2::guessing_game()?;

    // chapter3::common_concepts()?;

    // chapter4::ownership()?;

    // chapter5::main()?;

    // chapter6::main()?;

    // chapter8::main()?;

    chapter9::main()?;

    Ok(())
}
