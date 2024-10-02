// src/config.rs
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub log_file: String,
    pub temperature_threshold: f32,
    pub check_interval: u64,
}

impl Config {
    pub fn load(filename: &str) -> Config {
        let contents = fs::read_to_string(filename)
            .expect("Failed to read config file");
        toml::from_str(&contents).expect("Failed to parse config file")
    }
}