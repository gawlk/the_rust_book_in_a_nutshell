use color_eyre::Result;

#[allow(dead_code)]
pub fn ownership() -> Result<()> {
    // ---
    // Stack
    // ---

    //finite memory frames
    let _n = 5; // 1. [main, [[_n, 5]]]
    let _y = plus_one(_n); // 3. [main, [[_n, 5], [_y, 6]]]
    fn plus_one(x: i32) -> i32 {
        // 2. [main, [[_n, 5]]], [plus_one, [[x, 5]]]
        x + 1
    }
    // Array copy
    let _a = [0; 1_000_000]; // [main, [[a, [0, 0, 0, 0, 0,...]]]]
    let _b = _a; // [main, [[a, [0, 0, 0, 0, 0,...]], [b, [0, 0, 0, 0, 0,...]]]]

    // ---
    // Heap
    // ---

    // inifinite memory region (pointers)
    let _a = Box::new([0; 1_000]); // [main, [[a, heap[0]]]], heap [[0, 0, 0, 0, 0,...]]
    let _b = _a; // [main, [[a, heap[0]], [b, heap[0]]]], heap [[0, 0, 0, 0, 0,...]]

    // Stack types
    // numbers, booleans, arrays, tuples?

    // Heap types
    // Vec, String, Hashmap

    // ---
    // Ownership
    // ---

    let first = String::from("Ferris");
    let _second = first; // first moved to second, first cannot be used anymore
    let third = String::from("Ferris");
    #[allow(clippy::redundant_clone)]
    let _third_clone = third.clone(); // third is copied both are valid

    // ---
    // Borrowing
    // ---

    // note the ampersands
    fn greet(g1: &String, g2: &String) {
        println!("{} {}!", g1, g2);
    }
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let _s = format!("{} {}", m1, m2);

    let mut x: Box<i32> = Box::new(1);
    let _a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value,
             //     so x points to the value 2

    #[allow(clippy::borrowed_box)]
    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let _b: i32 = **r1; // two dereferences get us to the heap value

    #[allow(clippy::explicit_auto_deref)]
    let r2: &i32 = &*x; // r2 points to the heap value directly
    let _c: i32 = *r2; // so only one dereference is needed to read it

    // ---
    // Dereferencing
    // ---

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    println!("{} == {}", x_abs1, x_abs2);

    #[allow(clippy::borrowed_box)]
    let r: &Box<i32> = &x;
    let _r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let _r_abs2 = r.abs(); // implicit dereference (twice)

    let s = String::from("Hello");
    let _s_len1 = str::len(&s); // explicit reference
    let _s_len2 = s.len(); // implicit reference

    // ---
    // Borrow check
    // ---

    #[allow(unused_mut)]
    let mut vec: Vec<i32> = vec![1, 2, 3];
    // `vec` permissions: Read, Write, Own

    let num: &i32 = &vec[2];
    // `vec` permissions: Read, X, X
    // since it's a part of it is borrowed by `num`
    // `num` permissions: Read, X, Own
    // *num` permissions: Read, X, Own

    println!("Third element is {}", *num);
    // `vec` permissions: Read, Write, Own
    // `num` and `*num` permissions: X, X, X
    // since they're dropped

    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut vec[2]; // Because `num` is a mutable reference `vec` loses all rights, including Read, as long as `num` exists
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", vec);

    // Unsafe code
    let mut a = [0, 1, 2, 3];
    let (x, rest) = a.split_first_mut().unwrap();
    let y = &rest[0];
    *x += *y;
    // Could also be
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0] as *mut i32;
    let y = &a[1] as *const i32;
    unsafe {
        *x += *y;
    } // DO NOT DO THIS unless you know what you're doing!

    // Slice a String or a &str
    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("the first word is: {}", word);

    // Slice an array
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);

    Ok(())
}

// Slice: strings
// - v1
// fn first_word(s: &String) -> &str {
//     s.split(' ').collect::<Vec<_>>().first().unwrap_or(&"")
// }
// - v2
fn first_word(s: &str) -> &str {
    s.chars()
        .enumerate()
        .find_map(|(index, value)| if value == ' ' { Some(index) } else { None })
        .map_or(s, |index| &s[0..index])
}
