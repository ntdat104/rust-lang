mod flow_and_conditions;
mod functions_and_modules;
mod types_and_variables;

fn main() {
    variables_and_constants();
    data_types();
    types_and_variables::run();
    functions_and_modules::run();
    functions_and_modules::namehelpers::print();
    flow_and_conditions::run();
}

fn data_types() {
    // Unit Types
    let _x: () = ();
    println!("{:?}", _x);

    // Type conversion
    let x: f32 = 255.0;
    let y: u8 = x as u8 - 5;
    println!("{:?}", y);

    let guess: u32 = "24".parse().expect("Not a number!");
    println!("Parsed number: {guess}");

    // Scalar Types
    let a: i32 = -42; // Integer
    let b: f64 = 3.14; // Floating-point
    let c: bool = true; // Boolean
    let d: char = 'R'; // Character

    println!("Integer: {a}, Float: {b}, Boolean: {c}, Character: {d}");

    // Integer Types
    let _int8: i8 = -128;
    let _uint8: u8 = 255;
    let _int16: i16 = -32_768;
    let _uint16: u16 = 65_535;
    let _int32: i32 = -2_147_483_648;
    let _uint32: u32 = 4_294_967_295;
    let _int64: i64 = -9_223_372_036_854_775_808;
    let _uint64: u64 = 18_446_744_073_709_551_615;
    let _int128: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let _uint128: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let _isize_var: isize = -1; // Pointer-sized integer
    let _usize_var: usize = 1; // Pointer-sized unsigned integer

    println!("MIN(i8) = {}", std::i8::MIN);
    println!("MAX(i8) = {}", std::i8::MAX);

    let decimal: i32 = 98_222;
    let hex: i32 = 0xff;
    let octal: i32 = 0o77;
    let binary: i32 = 0b1111_0000;
    let byte: u8 = b'A';

    println!("Decimal: {decimal}, Hex: {hex}, Octal: {octal}, Binary: {binary}, Byte: {byte}");

    // Floating-Point Types
    let _float32: f32 = 2.5;
    let _float64: f64 = 3.141592653589793;

    // Numeric Operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    // The Boolean Type
    let t: bool = true;
    let f: bool = false;
    println!("Boolean values: t = {t}, f = {f}");

    // The Character Type
    let c1: char = 'z';
    let c2: char = 'ℤ';
    let c3: char = '😊';
    println!("Character values: c1 = {c1}, c2 = {c2}, c3 = {c3}");

    // Compound Types
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("Tuple values: x = {x}, y = {y}, z = {z}");
    println!(
        "Accessing tuple directly: first = {}, second = {}, third = {}",
        tuple.0, tuple.1, tuple.2
    );

    // The Array Type
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array values: {:?}", array);

    // Slice
    let new_array: &[i32] = &array[1..4];
    println!("{:?}", new_array);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Months: {:?}", months);

    let a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];
}

fn variables_and_constants() {
    // Variables and Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    // Shadowing with different types
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");
}
