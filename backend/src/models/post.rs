use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct IdOnlyPost {
    pub id: i32,
}
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct PagedPosts {
    pub offset: i64,
    pub limit: i64,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct NewPost {
    pub author: Option<String>,
    pub author_id: Option<i64>,
    pub content: Option<String>,
    pub hearts: Option<i64>,
    pub hearted_users: Option<Vec<i64>>,
}
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Post {
    pub id: i32,
    pub author: String,
    pub author_id: i64,
    pub content: String,
    pub hearts: i64,
    pub hearted_users: Vec<i64>,
    pub timestamp: DateTime<Utc>,
}

pub fn verify(content: Option<String>) -> bool {
    if let Some(c) = content {
        if c.replace(" ", "").is_empty() {
            return false;
        }
        if c.len() > 512 {
            return false;
        }
    } else {
        return false;
    }

    true
}
