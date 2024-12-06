use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub status: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub content: String,
    pub status: Option<i8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: Option<i8>,
}
