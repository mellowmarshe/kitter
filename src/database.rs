use crate::{
    constants,
    models::{
        github::User,
        post::{IdOnlyPost, NewPost, PagedPosts, Post},
    },
};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use sqlx::{migrate::Migrator, postgres::PgDone};
use std::error::Error;
use std::path::Path;

#[derive(Clone)]
pub struct ConnectionPool {
    pool: PgPool,
}

impl ConnectionPool {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&constants::CONFIG.database.postgres)
            .await?;

        Ok(Self { pool })
    }

    pub async fn migrations(&self) -> Result<(), sqlx::Error> {
        Migrator::new(Path::new("./migrations"))
            .await?
            .run(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn add_post(&self, post: &NewPost) -> Result<Post, sqlx::Error> {
        sqlx::query_as::<_, Post>(
            r#"INSERT INTO posts("content", "author", "author_id") VALUES ($1, $2, $3) RETURNING *"#,
        )
        .bind(post.content.clone())
        .bind(post.author.clone())
        .bind(post.author_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all_posts(&self) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY timestamp DESC")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_paged_posts(&self, paged: &PagedPosts) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY timestamp DESC OFFSET $1 LIMIT $2")
            .bind(paged.offset)
            .bind(paged.limit)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn delete_post(
        &self,
        post: &IdOnlyPost,
        user: &User,
    ) -> Result<IdOnlyPost, sqlx::Error> {
        sqlx::query_as::<_, IdOnlyPost>(
            "DELETE FROM posts WHERE id = $1 AND author_id = $2 RETURNING id",
        )
        .bind(post.id)
        .bind(user.id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_changed_usernames(&self, user: &User) -> Result<PgDone, sqlx::Error> {
        sqlx::query("UPDATE posts SET author = $1 WHERE author_id = $2 AND author != $1")
            .bind(user.login.clone())
            .bind(user.id)
            .execute(&self.pool)
            .await
    }
}
