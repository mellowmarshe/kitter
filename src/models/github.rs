use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize)]
pub struct IdOnlyGithub {
    pub id: i64,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub login: String,
    pub id: i64,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub bio: Option<String>,
}
