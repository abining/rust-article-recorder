use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: i32,
    pub author_id: Vec<u8>,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub is_public: i8, // MySQL Boolean is TinyInt(1)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub is_public: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_public: Option<bool>,
}
