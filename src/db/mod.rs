use sqlx::mysql::MySqlPool;
use std::env;

pub async fn init_pool() -> Result<MySqlPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPool::connect(&database_url).await
}

// 初始化数据库表
pub async fn init_db(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    // 创建用户表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id BIGINT PRIMARY KEY AUTO_INCREMENT,
            username VARCHAR(50) NOT NULL UNIQUE,
            email VARCHAR(255) NOT NULL UNIQUE,
            email_verified TINYINT NOT NULL DEFAULT 0,
            verification_code VARCHAR(6),
            verification_code_expires_at TIMESTAMP,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 创建文章表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS articles (
            id BIGINT PRIMARY KEY AUTO_INCREMENT,
            title VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            author_id BIGINT NOT NULL,
            status TINYINT NOT NULL DEFAULT 1 COMMENT '1: draft, 2: published',
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            FOREIGN KEY (author_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
