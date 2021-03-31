use crate::config::Config;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{env, path::PathBuf};

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::new(PathBuf::from(if env::var("PROD").is_ok() {
        "prod_config.toml"
    } else {
        "config.toml"
    }))
});
pub static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
    ).unwrap()
});
pub static USERNAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[a-zA-Z][a-zA-Z0-9-_]{3,32}"#).unwrap());

pub static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^[a-zA-Z0-9!"\#\\$%&'\(\)\*\+,-\.\\/:;<=>\?@\[\]\^_`\{\|}~]{8,}$"#).unwrap()
});
