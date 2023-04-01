use color_eyre::Result;
use std::{env, process};

use minigrep::{run, Config};

fn main() -> Result<()> {
    color_eyre::install()?;

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }

    Ok(())
}
