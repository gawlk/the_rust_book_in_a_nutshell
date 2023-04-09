#![allow(unused)]

use color_eyre::Result;

mod _box;
mod _deref;
mod _drop;
mod _rc;
mod _refcell;
mod _weak;

pub fn main() -> Result<()> {
    // _box::main()?;

    // _deref::main()?;

    // _drop::main()?;

    // _rc::main()?;

    // _refcell::main()?;

    _weak::main();

    Ok(())
}
