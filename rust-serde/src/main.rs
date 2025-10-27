use std::fs;

use serde::{Serialize, Deserialize};
use rust_serde::{to_json_pretty, save_to_file, load_from_file};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
    email: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        name: "Bob".to_string(),
        age: 25,
        email: "bob@example.com".to_string(),
    };

    // Serialize to JSON
    let json = to_json_pretty(&user)?;
    println!("Pretty JSON:\n{}", json);

    let file_name = "user.json";

    // Save to file
    save_to_file(&user, file_name)?;

    // Load from file
    let loaded_user: User = load_from_file(file_name)?;
    println!("Loaded: {:?}", loaded_user);

    fs::remove_file(file_name)?;

    Ok(())
}
