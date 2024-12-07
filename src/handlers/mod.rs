pub mod auth;
pub mod article;
pub mod app;

use actix_web::{get, HttpResponse, Responder};

pub use auth::*;
pub use article::*;
pub use app::*;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("OK")
}
