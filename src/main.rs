use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_web::{HttpResponse, http::StatusCode};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;

mod config;
mod middleware;
mod models;
mod handlers;
mod db;
mod utils;

use crate::config::auth::JwtConfig;
use crate::utils::email::EmailService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv().ok();
    env_logger::init();

    // 获取配置
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 创建数据库连接池
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // 创建 JWT 配置
    let jwt_config = JwtConfig::from_env();

    // 创建邮件服务
    let email_service = EmailService::new();

    // 共享数据库连接池
    let db_pool = web::Data::new(pool);

    log::info!("Starting server at http://{}:{}", host, port);

    // 创建并启动 HTTP 服务器
    HttpServer::new(move || {
        // 配置 CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(db_pool.clone())
            .app_data(web::Data::new(jwt_config.clone()))
            .app_data(web::Data::new(email_service.clone()))
            .service(
                web::scope("/api")
                    .service(handlers::health_check)
                    .service(handlers::register)
                    .service(handlers::login)
                    .service(handlers::get_verification_code)
                    .service(
                        web::scope("/apps")
                            .route("", web::post().to(handlers::create_app))
                            .route("", web::get().to(handlers::list_apps))
                            .route("/{id}", web::get().to(handlers::get_app))
                            .route("/{id}", web::put().to(handlers::update_app))
                            .route("/{id}", web::delete().to(handlers::delete_app))
                    )
            )
            .service(handlers::auth::me)
            .service(handlers::article::create_article)
            .service(handlers::article::get_article)
            .service(handlers::article::list_articles)
            .service(handlers::article::update_article)
            .service(handlers::article::delete_article)
            // 404 处理
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound()
                    .json(serde_json::json!({
                        "code": StatusCode::NOT_FOUND.as_u16(),
                        "message": "Resource not found"
                    }))
            }))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
