#![allow(unused)]
use color_eyre::Result;

pub fn main() -> Result<()> {
    // The rest is in lib.rs

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
