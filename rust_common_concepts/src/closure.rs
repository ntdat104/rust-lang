struct Person {
    name: String,
    age: u8,
}

pub fn run() {
    let closure_example = || println!("This is a closure example.");
    closure_example();

    let closure_example_2 = |x| println!("This is another closure example. {}", x);
    closure_example_2(42);

    let sum = |x: u8, y: u8| x + y;
    println!("Sum of 2 and 3 is {}", sum(2, 3));

    let mut p1 = Person {
        name: "Bob".to_string(),
        age: 30,
    };
    let mut change_name = |age| {
        p1.name = "Alice".to_string();
        p1.age = age;
    };
    change_name(18);
    println!("Name changed to {}, age changed to {}", p1.name, p1.age);

    // change_name(23); // error borrow check
}
