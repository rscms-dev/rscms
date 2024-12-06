use crate::middleware::auth::AuthenticatedUser;
use crate::models::article::{CreateArticleRequest, UpdateArticleRequest};
use crate::models::{Article, MessageResponse};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::MySqlPool;

#[post("/articles")]
pub async fn create_article(
    pool: web::Data<MySqlPool>,
    article: web::Json<CreateArticleRequest>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let status = article.status.unwrap_or(1); // 默认为草稿状态
    
    let result = sqlx::query!(
        r#"
        INSERT INTO articles (title, content, author_id, status)
        VALUES (?, ?, ?, ?)
        "#,
        article.title,
        article.content,
        auth_user.user_id,
        status
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            // 获取新创建的文章
            let article = sqlx::query_as!(
                Article,
                "SELECT * FROM articles WHERE id = ?",
                result.last_insert_id()
            )
            .fetch_one(pool.get_ref())
            .await;

            match article {
                Ok(article) => HttpResponse::Ok().json(article),
                Err(e) => {
                    log::error!("Failed to fetch created article: {:?}", e);
                    HttpResponse::InternalServerError().json(MessageResponse {
                        message: "Article created but failed to fetch it".to_string(),
                    })
                }
            }
        }
        Err(e) => {
            log::error!("Failed to create article: {:?}", e);
            HttpResponse::InternalServerError().json(MessageResponse {
                message: "Failed to create article".to_string(),
            })
        }
    }
}

#[get("/articles/{id}")]
pub async fn get_article(
    pool: web::Data<MySqlPool>,
    article_id: web::Path<i64>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let result = sqlx::query_as!(
        Article,
        r#"
        SELECT * FROM articles 
        WHERE id = ? AND (status = 2 OR author_id = ?)
        "#,
        article_id.into_inner(),
        auth_user.user_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(article)) => HttpResponse::Ok().json(article),
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "Article not found".to_string(),
        }),
        Err(e) => {
            log::error!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(MessageResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[get("/articles")]
pub async fn list_articles(
    pool: web::Data<MySqlPool>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let result = sqlx::query_as!(
        Article,
        r#"
        SELECT * FROM articles 
        WHERE status = 2 OR author_id = ?
        ORDER BY created_at DESC
        "#,
        auth_user.user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(e) => {
            log::error!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(MessageResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[put("/articles/{id}")]
pub async fn update_article(
    pool: web::Data<MySqlPool>,
    article_id: web::Path<i64>,
    article: web::Json<UpdateArticleRequest>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let article_id = article_id.into_inner();
    
    // 首先检查文章是否存在且属于当前用户
    let existing = sqlx::query!(
        "SELECT id FROM articles WHERE id = ? AND author_id = ?",
        article_id,
        auth_user.user_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match existing {
        Ok(Some(_)) => {
            let mut query_parts = Vec::new();
            let mut query_values = Vec::new();

            if let Some(title) = &article.title {
                query_parts.push("title = ?");
                query_values.push(title.clone());
            }
            if let Some(content) = &article.content {
                query_parts.push("content = ?");
                query_values.push(content.clone());
            }
            if let Some(status) = article.status {
                query_parts.push("status = ?");
                query_values.push(status.to_string());
            }

            if query_parts.is_empty() {
                return HttpResponse::BadRequest().json(MessageResponse {
                    message: "No fields to update".to_string(),
                });
            }

            let query = format!(
                "UPDATE articles SET {} WHERE id = ? AND author_id = ?",
                query_parts.join(", ")
            );

            let mut db_query = sqlx::query(&query);
            
            // 绑定所有参数
            for value in query_values {
                db_query = db_query.bind(value);
            }
            
            // 绑定 WHERE 子句的参数
            db_query = db_query.bind(article_id).bind(auth_user.user_id);

            let result = db_query.execute(pool.get_ref()).await;

            match result {
                Ok(_) => {
                    // 获取更新后的文章
                    let updated = sqlx::query_as!(
                        Article,
                        "SELECT * FROM articles WHERE id = ? AND author_id = ?",
                        article_id,
                        auth_user.user_id
                    )
                    .fetch_one(pool.get_ref())
                    .await;

                    match updated {
                        Ok(article) => HttpResponse::Ok().json(article),
                        Err(e) => {
                            log::error!("Failed to fetch updated article: {:?}", e);
                            HttpResponse::InternalServerError().json(MessageResponse {
                                message: "Article updated but failed to fetch it".to_string(),
                            })
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to update article: {:?}", e);
                    HttpResponse::InternalServerError().json(MessageResponse {
                        message: "Failed to update article".to_string(),
                    })
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "Article not found or you don't have permission to update it".to_string(),
        }),
        Err(e) => {
            log::error!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(MessageResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[delete("/articles/{id}")]
pub async fn delete_article(
    pool: web::Data<MySqlPool>,
    article_id: web::Path<i64>,
    auth_user: AuthenticatedUser,
) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM articles WHERE id = ? AND author_id = ?",
        article_id.into_inner(),
        auth_user.user_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().json(MessageResponse {
                    message: "Article deleted successfully".to_string(),
                })
            } else {
                HttpResponse::NotFound().json(MessageResponse {
                    message: "Article not found or you don't have permission to delete it".to_string(),
                })
            }
        }
        Err(e) => {
            log::error!("Failed to delete article: {:?}", e);
            HttpResponse::InternalServerError().json(MessageResponse {
                message: "Failed to delete article".to_string(),
            })
        }
    }
}
