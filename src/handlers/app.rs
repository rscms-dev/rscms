use actix_web::{web, HttpResponse, Responder};
use sqlx::{MySqlPool, Row};
use crate::models::{App, CreateAppRequest, UpdateAppRequest, AppQuery, AppResponse, AppListResponse, MessageResponse};
use crate::middleware::auth::AuthenticatedUser;

// 创建应用
pub async fn create_app(
    pool: web::Data<MySqlPool>,
    user: AuthenticatedUser,
    req: web::Json<CreateAppRequest>,
) -> impl Responder {
    // 检查应用标识是否已存在
    let exists = sqlx::query!("SELECT id FROM apps WHERE identifier = ?", req.identifier)
        .fetch_optional(pool.get_ref())
        .await;

    match exists {
        Ok(Some(_)) => return HttpResponse::BadRequest().json(MessageResponse {
            message: "应用标识已存在".to_string(),
        }),
        Ok(None) => {},
        Err(e) => return HttpResponse::InternalServerError().json(MessageResponse {
            message: format!("检查应用标识失败: {}", e),
        }),
    }

    let result = sqlx::query(
        r#"
        INSERT INTO apps (name, description, identifier, creator_id, updater_id)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.identifier)
    .bind(user.user_id)
    .bind(user.user_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(MessageResponse {
            message: "应用创建成功".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(MessageResponse {
            message: format!("创建应用失败: {}", e),
        }),
    }
}

// 更新应用
pub async fn update_app(
    pool: web::Data<MySqlPool>,
    user: AuthenticatedUser,
    path: web::Path<i64>,
    req: web::Json<UpdateAppRequest>,
) -> impl Responder {
    let app_id = path.into_inner();
    
    let mut query = String::from("UPDATE apps SET updater_id = ?");
    let mut params: Vec<String> = vec![];
    
    if let Some(name) = &req.name {
        query.push_str(", name = ?");
        params.push(name.clone());
    }
    
    if let Some(description) = &req.description {
        query.push_str(", description = ?");
        params.push(description.clone());
    }
    
    query.push_str(" WHERE id = ?");
    
    let mut db_query = sqlx::query(&query)
        .bind(user.user_id);
    
    for param in params {
        db_query = db_query.bind(param);
    }
    
    db_query = db_query.bind(app_id);
    
    match db_query.execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(MessageResponse {
            message: "应用更新成功".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(MessageResponse {
            message: format!("更新应用失败: {}", e),
        }),
    }
}

// 删除应用
pub async fn delete_app(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> impl Responder {
    let app_id = path.into_inner();
    
    match sqlx::query("DELETE FROM apps WHERE id = ?")
        .bind(app_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(MessageResponse {
            message: "应用删除成功".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(MessageResponse {
            message: format!("删除应用失败: {}", e),
        }),
    }
}

// 获取应用列表
pub async fn list_apps(
    pool: web::Data<MySqlPool>,
    query: web::Query<AppQuery>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;
    
    let mut sql = String::from("SELECT * FROM apps WHERE 1=1");
    let mut count_sql = String::from("SELECT COUNT(*) as count FROM apps WHERE 1=1");
    let mut params = Vec::new();
    let mut count_params = Vec::new();
    
    if let Some(keyword) = &query.keyword {
        sql.push_str(" AND name LIKE ?");
        count_sql.push_str(" AND name LIKE ?");
        let pattern = format!("%{}%", keyword);
        params.push(pattern.clone());
        count_params.push(pattern);
    }
    
    if let Some(identifier) = &query.identifier {
        sql.push_str(" AND identifier = ?");
        count_sql.push_str(" AND identifier = ?");
        params.push(identifier.to_string());
        count_params.push(identifier.to_string());
    }
    
    if let Some(creator_id) = query.creator_id {
        sql.push_str(" AND creator_id = ?");
        count_sql.push_str(" AND creator_id = ?");
        params.push(creator_id.to_string());
        count_params.push(creator_id.to_string());
    }
    
    sql.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
    
    let mut db_query = sqlx::query_as::<_, App>(&sql);
    let mut count_query = sqlx::query(&count_sql);
    
    for param in &params {
        db_query = db_query.bind(param);
    }
    
    for param in &count_params {
        count_query = count_query.bind(param);
    }
    
    db_query = db_query.bind(page_size).bind(offset);
    
    let apps = match db_query.fetch_all(pool.get_ref()).await {
        Ok(apps) => apps,
        Err(e) => return HttpResponse::InternalServerError().json(MessageResponse {
            message: format!("获取应用列表失败: {}", e),
        }),
    };
    
    let count: i64 = match count_query
        .fetch_one(pool.get_ref())
        .await
        .and_then(|row| row.try_get("count"))
    {
        Ok(count) => count,
        Err(e) => return HttpResponse::InternalServerError().json(MessageResponse {
            message: format!("获取应用总数失败: {}", e),
        }),
    };
    
    let response = AppListResponse {
        apps: apps.into_iter().map(|t| AppResponse {
            id: t.id,
            name: t.name,
            description: t.description,
            identifier: t.identifier,
            creator_id: t.creator_id,
            created_at: t.created_at,
            updater_id: t.updater_id,
            updated_at: t.updated_at,
        }).collect(),
        total: count,
        page,
        page_size,
    };
    
    HttpResponse::Ok().json(response)
}

// 获取单个应用
pub async fn get_app(
    pool: web::Data<MySqlPool>,
    path: web::Path<i64>,
) -> impl Responder {
    let app_id = path.into_inner();
    
    match sqlx::query_as::<_, App>("SELECT * FROM apps WHERE id = ?")
        .bind(app_id)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(Some(app)) => HttpResponse::Ok().json(AppResponse {
            id: app.id,
            name: app.name,
            description: app.description,
            identifier: app.identifier,
            creator_id: app.creator_id,
            created_at: app.created_at,
            updater_id: app.updater_id,
            updated_at: app.updated_at,
        }),
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "应用不存在".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(MessageResponse {
            message: format!("获取应用失败: {}", e),
        }),
    }
}
