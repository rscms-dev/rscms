use std::env;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use route::index::general_routes;

#[path = "./api/mod.rs"]
mod api;
#[path = "./route/mod.rs"]
mod route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_url = env::var("RSCMS_MONGODB_URL");
    if mongo_url.is_err() {
        panic!("MongoDB URL not set!");
    }
    println!("MongoDB URL: {}", mongo_url.unwrap());

    let app = move || App::new().configure(general_routes);
    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}

#[cfg(test)]
mod tests {
    use bcrypt::{hash, verify, DEFAULT_COST};

    #[test]
    fn test_default_password_match() {
        let passwd = "rscms-admin";
        let hashed = hash(passwd, DEFAULT_COST).unwrap();
        println!("hashed: {}", hashed);

        let valid = verify(passwd, &hashed).unwrap();
        assert!(valid);
    }
}
