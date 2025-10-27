use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub api_key: String,
    pub secret_key: String,
    pub endpoint: String,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let raw = fs::read_to_string(path).expect("Failed to read config.toml");
        let data: toml::Value = toml::from_str(&raw).expect("Invalid TOML format");

        let api_key = data["api_key"].as_str().unwrap().to_string();
        let secret_key = data["secret_key"].as_str().unwrap().to_string();
        let endpoint = data["binance"]["endpoint"].as_str().unwrap().to_string();

        Config { api_key, secret_key, endpoint }
    }
}
