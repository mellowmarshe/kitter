use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Auth {
    pub token: String,
}

#[derive(Deserialize)]
pub struct Callback {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub scope: String,
}
