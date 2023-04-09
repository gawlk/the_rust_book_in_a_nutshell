#![allow(unused, clippy::deref_addrof)]

use color_eyre::Result;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn main() -> Result<()> {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    let e = CustomSmartPointer {
        data: String::from("otherrrrr stuff"),
    };

    println!("CustomSmartPointers created.");

    drop(e);

    println!("CustomSmartPointer dropped before the end of main.");

    Ok(())
}
