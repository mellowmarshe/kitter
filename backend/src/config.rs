use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub postgres: String,
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        config
    }
}
