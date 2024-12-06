pub mod email;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            AppError::DatabaseError(_) => {
                actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal server error"
                }))
            }
            AppError::ValidationError(msg) => {
                actix_web::HttpResponse::BadRequest().json(serde_json::json!({
                    "error": msg
                }))
            }
            AppError::NotFound(msg) => {
                actix_web::HttpResponse::NotFound().json(serde_json::json!({
                    "error": msg
                }))
            }
            AppError::Unauthorized(msg) => {
                actix_web::HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": msg
                }))
            }
        }
    }
}
