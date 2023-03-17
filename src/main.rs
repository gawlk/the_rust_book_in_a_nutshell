use color_eyre::eyre::Result;

mod chapter2;
mod chapter3;
mod chapter4;
mod chapter5;
mod chapter6;

fn main() -> Result<()> {
    color_eyre::install()?;

    // chapter2::guessing_game()?;

    // chapter3::common_concepts()?;

    // chapter4::ownership()?;

    // chapter5::main()?;

    chapter6::main()?;

    Ok(())
}
