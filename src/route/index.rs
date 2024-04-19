use actix_web::{web, HttpResponse, Responder};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(|| async { "Hello" }));
    cfg.route("/health", web::get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("ok")
}
