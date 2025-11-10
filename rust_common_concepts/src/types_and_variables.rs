pub fn run() {
    // Unit type
    let x: () = ();
    println!("{:?}", x);

    // Type conversion
    let x: f32 = 255.0;
    let y: u8 = x as u8 - 5;
    println!("{:?}", y);

    // Parse a string to a number
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

    // String
    let mut first_name: &str = "Alice";
    println!("{}", first_name);
    first_name = "Bob";
    println!("{}", first_name);

    // Tuple
    let name = ("Alice", "Bob", 40);
    // println!("{}", name); - error vì kiểu tuple không hỗ trợ print
    println!("{:?}", name);
    println!("{}", name.0);
    let (name1, name2, age) = name;
    println!("{name1} {name2} {age}");

    // Array
    let ages: [u16; 6] = [30, 45, 50, 55, 60, 65];
    // println!("{}", ages); - error vì kiểu array không hỗ trợ print
    println!("{:?}", ages);

    // Slice
    let new_ages = &ages[1..=4];
    println!("{:?}", new_ages);
}
