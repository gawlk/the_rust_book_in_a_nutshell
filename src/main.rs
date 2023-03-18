// Point of entry for binary crates

use color_eyre::eyre::Result;

mod chapters;
use chapters::*;

fn main() -> Result<()> {
    color_eyre::install()?;

    // chapter2::guessing_game()?;

    // chapter3::common_concepts()?;

    // chapter4::ownership()?;

    // chapter5::main()?;

    chapter6::main()?;

    Ok(())
}
