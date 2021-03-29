use crate::constants;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Register {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct UserPassword {
    pub username: String,
    pub password: String,
}

pub fn verify(user: Register) -> bool {
    if user.username.len() > 32
        || user.username.len() < 3
        || !&constants::USERNAME_REGEX.is_match(&user.username)
    {
        return false;
    }

    if !&constants::EMAIL_REGEX.is_match(&user.email) {
        return false;
    }

    if !&constants::PASSWORD_REGEX.is_match(&user.password) {
        return false;
    }

    return true;
}
