use crate::config::Config;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new(PathBuf::from("config.toml")));
