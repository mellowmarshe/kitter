use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub github: GithubConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
    pub secured: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub postgres: String,
    pub redis: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GithubConfig {
    pub client_id: String,
    pub client_secret: String,
    pub state: String,
    pub redirect: String,
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        config
    }
}
