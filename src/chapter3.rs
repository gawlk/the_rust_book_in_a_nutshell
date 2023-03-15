use color_eyre::Result;

#[allow(dead_code)]
pub fn common_concepts() -> Result<()> {
    // Immutable
    let x = 5;
    println!("The value of x is: {x}");

    // Shadowing
    let mut x = 6;
    println!("The value of x is: {x}");
    // Shadowing bis
    {
        // Only shadows _x is this scope
        let _x = 6.6;
        let _x = "_x";
    }

    // Mutated
    x = 7;
    println!("The value of x is: {x}");

    // Const
    #[allow(dead_code)]
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Integers
    let _i: i8 = 0; // -128 ..= 127
    let _i: u8 = 0; // 0 ..= 255
    let _i: i16 = 0; // -32_768 ..= 32_767
    let _i: u16 = 0; // 0 ..= 65_535
    let _i: i32 = 0; // -2_147_483_648 ..= 2_147_483_647
    let _i: u32 = 0; // 0 ..= 4_294_967_295
    let _i: i64 = 0; // -9_223_372_036_854_775_808 ..= 9_223_372_036_854_775_807
    let _i: u64 = 0; // 0 ..= 18_446_744_073_709_551_615
    let _i: isize = 0; // i32 or i64
    let _i: usize = 0; // u32 or u64
    let _hex = 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let _byte = b'A';

    // Floats
    let _f: f32 = 0.111_111_11; // 8
    let _f: f64 = 0.111_111_111_111_111_1; // 16

    // Boolean
    let _b: bool = true;

    // Characters
    let _c: char = 'c';
    let _c: char = '🫥';

    // Tuples (fixed size with mixed types)
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let _tup = (500, 6.4, 1);
    let (_x, _y, _z) = _tup;
    let _five_hundred = _tup.1;

    // Unit (an empty value), a special Tuple
    #[allow(clippy::let_unit_value)]
    let _unit = ();

    // Arrays (fized size with same types)
    let _a = [1, 2, 3, 4, 5];
    let _five_threes = [3; 5];
    let _first_three = _five_threes[0];

    // Functions
    fn _print_number(n: i32) {
        println!("n is {n}");
    }
    fn _return_n_plus_1(n: i32) -> i32 {
        n + 1
    }

    // Expressions
    let _x = {
        let y = x;
        y + 1
    };

    // Ifs
    let number = 3;
    // Conditions must be booleans
    if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let _number = if true { 5 } else { 6 };

    // Loops (infinite)
    #[allow(clippy::never_loop)]
    loop {
        break;
    }
    // Loop with return
    let mut counter = 0;
    let _result = loop {
        counter += 1;
        if counter == 10 {
            continue; // next iteration
        }
        if counter >= 10 {
            break counter * 2; // skips 10 so returns 11 * 2
        }
    };
    // Loop labels (for "break" and "continue")
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break; // Break closest loop
            }
            if count == 2 {
                break 'counting_up; // Break loop with label
            }
            remaining -= 1;
        }
        count += 1;
    }

    // While
    let mut number = 3;
    while number != 0 {
        number -= 1;
    }

    // For
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    Ok(())
}
