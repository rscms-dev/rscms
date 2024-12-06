use crate::config::auth::{Claims, JwtConfig};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{AuthResponse, LoginRequest, MessageResponse, RegisterRequest, User};
use crate::utils::email::EmailService;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::encode;
use rand::Rng;
use sqlx::MySqlPool;

// 生成6位验证码
fn generate_verification_code() -> String {
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(0..1000000))
}

#[post("/auth/register")]
pub async fn register(
    pool: web::Data<MySqlPool>,
    user: web::Json<RegisterRequest>,
    email_service: web::Data<EmailService>,
) -> impl Responder {
    // 检查用户是否已存在
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = ?",
        user.email
    )
    .fetch_optional(pool.get_ref())
    .await;

    match existing_user {
        Ok(Some(_)) => {
            HttpResponse::BadRequest().json(MessageResponse {
                message: "User with this email already exists".to_string(),
            })
        }
        Ok(None) => {
            // 生成验证码
            let verification_code: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(6)
                .map(char::from)
                .collect();

            // 设置验证码过期时间（30分钟后）
            let expires_at = Utc::now() + Duration::minutes(30);

            // 创建新用户
            let result = sqlx::query!(
                r#"
                INSERT INTO users (username, email, email_verified, verification_code, verification_code_expires_at)
                VALUES (?, ?, ?, ?, ?)
                "#,
                user.username,
                user.email,
                0i8, // false
                verification_code,
                expires_at
            )
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => {
                    // 发送验证码邮件
                    if let Err(e) = email_service.send_verification_code(&user.email, &verification_code) {
                        log::error!("Failed to send verification email: {:?}", e);
                        return HttpResponse::InternalServerError().json(MessageResponse {
                            message: "Failed to send verification email".to_string(),
                        });
                    }

                    HttpResponse::Ok().json(MessageResponse {
                        message: "Registration successful. Please check your email for verification code.".to_string(),
                    })
                }
                Err(e) => {
                    log::error!("Failed to create user: {:?}", e);
                    HttpResponse::InternalServerError().json(MessageResponse {
                        message: "Failed to create user".to_string(),
                    })
                }
            }
        }
        Err(e) => {
            log::error!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(MessageResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[post("/auth/login")]
pub async fn login(
    pool: web::Data<MySqlPool>,
    login_data: web::Json<LoginRequest>,
    jwt_config: web::Data<JwtConfig>,
) -> impl Responder {
    // 验证用户和验证码
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users 
        WHERE email = ? 
        AND verification_code = ? 
        AND verification_code_expires_at > CURRENT_TIMESTAMP
        "#,
        login_data.email,
        login_data.verification_code
    )
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(user)) => {
            // 更新用户状态为已验证
            let update_result = sqlx::query!(
                r#"
                UPDATE users 
                SET email_verified = ?, 
                    verification_code = NULL, 
                    verification_code_expires_at = NULL 
                WHERE id = ?
                "#,
                1i8, // true
                user.id
            )
            .execute(pool.get_ref())
            .await;

            match update_result {
                Ok(_) => {
                    // 生成 JWT token
                    let claims = Claims {
                        sub: user.id.to_string(),
                        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
                    };

                    match encode(
                        &jsonwebtoken::Header::default(),
                        &claims,
                        &jsonwebtoken::EncodingKey::from_secret(jwt_config.secret.as_bytes()),
                    ) {
                        Ok(token) => HttpResponse::Ok().json(AuthResponse { token, user }),
                        Err(e) => {
                            log::error!("Failed to create JWT token: {:?}", e);
                            HttpResponse::InternalServerError().json(MessageResponse {
                                message: "Failed to create authentication token".to_string(),
                            })
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to update user: {:?}", e);
                    HttpResponse::InternalServerError().json(MessageResponse {
                        message: "Failed to update user status".to_string(),
                    })
                }
            }
        }
        Ok(None) => HttpResponse::BadRequest().json(MessageResponse {
            message: "Invalid email or verification code".to_string(),
        }),
        Err(e) => {
            log::error!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(MessageResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[get("/auth/me")]
pub async fn me(pool: web::Data<MySqlPool>, auth_user: AuthenticatedUser) -> impl Responder {
    let user_info = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = ?",
        auth_user.user_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match user_info {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "User not found".to_string(),
        }),
        Err(e) => {
            log::error!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(MessageResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}
