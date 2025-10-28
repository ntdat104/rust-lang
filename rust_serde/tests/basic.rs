use serde::{Serialize, Deserialize};
use std::fs;

use rust_serde::{from_json, load_from_file, save_to_file, to_json, to_json_pretty};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
    email: String,
}

#[test]
fn test_to_json_and_from_json() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    let json = to_json(&user).unwrap();
    assert!(json.contains("Alice"));

    let deserialized: User = from_json(&json).unwrap();
    assert_eq!(user, deserialized);
}

#[test]
fn test_to_json_pretty_format() {
    let user = User {
        name: "Bob".to_string(),
        age: 25,
        email: "bob@example.com".to_string(),
    };

    let pretty = to_json_pretty(&user).unwrap();

    // should contain newlines for pretty format
    assert!(pretty.contains('\n'));
    assert!(pretty.contains("Bob"));
}

#[test]
fn test_save_and_load_file() {
    let user = User {
        name: "Charlie".to_string(),
        age: 40,
        email: "charlie@example.com".to_string(),
    };

    let path = "test_user.json";

    // Save
    save_to_file(&user, path).unwrap();
    assert!(fs::metadata(path).is_ok());

    // Load
    let loaded: User = load_from_file(path).unwrap();
    assert_eq!(user, loaded);

    // Cleanup
    fs::remove_file(path).unwrap();
}
