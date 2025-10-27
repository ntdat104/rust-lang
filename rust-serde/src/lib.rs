use serde::{Serialize, de::DeserializeOwned};
use std::{fs, error::Error};

/// Serialize any struct into a compact JSON string
pub fn to_json<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(data)
}

/// Serialize any struct into a pretty-formatted JSON string
pub fn to_json_pretty<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(data)
}

/// Deserialize a JSON string into a struct
pub fn from_json<T: DeserializeOwned>(data: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(data)
}

/// Save any serializable struct to a JSON file
pub fn save_to_file<T: Serialize>(data: &T, path: &str) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string(data)?;
    fs::write(path, json)?;
    Ok(())
}

/// Load a struct from a JSON file
pub fn load_from_file<T: DeserializeOwned>(path: &str) -> Result<T, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let result = serde_json::from_str(&data)?;
    Ok(result)
}
