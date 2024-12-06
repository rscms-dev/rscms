use sqlx::mysql::MySqlPool;
use std::env;

pub async fn init_pool() -> Result<MySqlPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPool::connect(&database_url).await
}

// 初始化数据库表
pub async fn init_db(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id BIGINT PRIMARY KEY AUTO_INCREMENT,
            username VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL,
            email_verified TINYINT(1) NOT NULL DEFAULT 0,
            verification_code VARCHAR(6),
            verification_code_expires_at TIMESTAMP NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE KEY unique_email (email),
            UNIQUE KEY unique_username (username)
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
