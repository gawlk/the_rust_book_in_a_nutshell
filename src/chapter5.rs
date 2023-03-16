use color_eyre::Result;

// Struct
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Tuple struct
struct Color(i32, i32, i32);

// Unit like struct
struct AlwaysEqual;

#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        match (self.width >= self.height, other.width >= other.height) {
            (true, true) | (false, false) => {
                self.width >= other.width && self.height >= other.height
            }
            (true, false) | (false, true) => {
                self.width >= other.height && self.height >= other.width
            }
        }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    // Won't work with current implementation of `max` since it needs to take ownership of `self`
    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}

pub fn main() -> Result<()> {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    user1.email = String::from("anotheremail@example.com");

    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let _black = Color(0, 0, 0);

    let _subject = AlwaysEqual;

    let mut p = Point { x: 0, y: 0 };
    let x = &mut p.x;
    *x += 1;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("{:?}", rect1);
    println!("{:#?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let mut square = Rectangle::square(4);

    println!("{:#?}", square);

    square.set_width(4);

    Ok(())
}
