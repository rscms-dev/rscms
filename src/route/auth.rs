use actix_web::web;

use crate::handler::auth::login;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/login", web::post().to(login));
}
