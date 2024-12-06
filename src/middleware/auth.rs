use crate::config::auth::{Claims, JwtConfig};
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Validation};

pub struct AuthenticatedUser {
    pub user_id: i64,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(header) => header,
            None => return err(ErrorUnauthorized("No authorization header")),
        };

        let auth_str = match auth_header.to_str() {
            Ok(str) => str,
            Err(_) => return err(ErrorUnauthorized("Invalid authorization header")),
        };

        if !auth_str.starts_with("Bearer ") {
            return err(ErrorUnauthorized("Invalid authorization header format"));
        }

        let token = &auth_str[7..];
        let jwt_config = JwtConfig::from_env();

        match decode::<Claims>(
            token,
            &jwt_config.decoding_key,
            &Validation::default(),
        ) {
            Ok(token_data) => {
                match token_data.claims.sub.parse::<i64>() {
                    Ok(user_id) => ok(AuthenticatedUser { user_id }),
                    Err(_) => err(ErrorUnauthorized("Invalid user ID in token")),
                }
            }
            Err(_) => err(ErrorUnauthorized("Invalid token")),
        }
    }
}
