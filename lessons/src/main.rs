// Point of entry for binary crates

use color_eyre::eyre::Result;

mod chapters;

fn main() -> Result<()> {
    color_eyre::install()?;

    chapters::main()?;

    Ok(())
}
