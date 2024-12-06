use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-super-secret-and-ultra-long-secret-key".to_string());
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            secret,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user_id as string
    pub exp: usize,   // expiration time as usize
}
