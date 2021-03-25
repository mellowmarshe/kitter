use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct IdOnlyPost {
    pub id: i32,
}
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct NewPost {
    pub author: Option<String>,
    pub author_id: Option<i64>,
    pub content: Option<String>,
}
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Post {
    pub id: i32,
    pub author: String,
    pub author_id: i64,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct PostWithOwner {
    pub id: i32,
    pub author: String,
    pub author_id: i64,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

pub fn verify(content: Option<String>) -> bool {
    if content.is_none() {
        return false;
    }
    if content.unwrap().len() > 512 {
        return false;
    }

    true
}
