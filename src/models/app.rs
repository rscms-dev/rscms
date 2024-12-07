use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct App {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub identifier: String,  // 应用标识，用于唯一标识一个应用
    pub creator_id: i64,
    pub created_at: DateTime<Utc>,
    pub updater_id: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppRequest {
    pub name: String,
    pub description: String,
    pub identifier: String,  // 应用标识
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppQuery {
    pub keyword: Option<String>,
    pub identifier: Option<String>,  // 按应用标识搜索
    pub creator_id: Option<i64>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct AppResponse {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub identifier: String,
    pub creator_id: i64,
    pub created_at: DateTime<Utc>,
    pub updater_id: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AppListResponse {
    pub apps: Vec<AppResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}
