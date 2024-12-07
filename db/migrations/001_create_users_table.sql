-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT 'Unique identifier',
    username VARCHAR(255) NOT NULL COMMENT 'Username',
    email VARCHAR(255) NOT NULL COMMENT 'Email',
    email_verified BOOLEAN NOT NULL DEFAULT FALSE COMMENT 'Email verified status',
    verification_code VARCHAR(6) NULL COMMENT 'Email verification code',
    verification_code_expires_at TIMESTAMP NULL COMMENT 'Email verification code expiration time',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Created time',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'Updated time',
    UNIQUE KEY unique_email (email) COMMENT 'Unique email',
    UNIQUE KEY unique_username (username) COMMENT 'Unique username'
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT 'Users table';

CREATE TABLE IF NOT EXISTS articles (
    id BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT 'Unique identifier',
    title VARCHAR(255) NOT NULL COMMENT 'Article title',
    content TEXT NOT NULL COMMENT 'Article content',
    author_id BIGINT NOT NULL COMMENT 'Author ID',
    status TINYINT NOT NULL DEFAULT 1 COMMENT 'Status: 1 - draft, 2 - published',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Created time',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'Updated time'
) COMMENT 'Articles table';