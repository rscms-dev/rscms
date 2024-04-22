use crate::handler::ApiResult;

use actix_web::{web::Json, Responder};

pub async fn login() -> impl Responder {
    Json(ApiResult::success("ok"))
}