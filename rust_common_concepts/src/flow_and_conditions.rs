pub fn run() {
    let age_to_drive = 16u8;

    println!("Enter the person's age:");
    let myinput = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();

    let age = myinput.replace("\n", "").parse::<u8>().unwrap();
    if age > age_to_drive {
        println!("Issuing driver's license, because they are old enough");
    } else if age == 16 || age > 14 {
        println!("You are just on the verge of being old enough! Wait one more year.");
    } else {
        println!("Wait a bit longer, you aren't old enough for a driver's license!");
    }

    let drivers_license = if age >= 16 { true } else { false };
    println!("Drivers license: {drivers_license}");
}

pub fn run_while() {
    let age_to_drive = 16u8;

    let mut current_age = 0u8;
    while current_age < age_to_drive {
        println!("Waiting {current_age} ...");
        current_age += 1;
        if current_age == 6 {
            break;
        }
    }
}

pub fn run_loop() {
    let mut x = 1;
    loop {
        println!("Hello from Rust!");
        if x > 5 {
            break;
        }
        x += 1;
    }
}

pub fn run_for_loop() {
    let ages = [18, 26, 35, 41];
    let age_to_drive = 20i32;

    for value in ages {
        println!("The current age is {0}", value);
        if value >= age_to_drive {
            println!("You are old enough to drive!");
        } else {
            println!("You need to wait a little bit more...");
        }
    }
}
