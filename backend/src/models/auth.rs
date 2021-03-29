use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Deserialize)]
pub struct Callback {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub company: String,
    pub exp: i64,
    pub iss: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub scope: String,
}
