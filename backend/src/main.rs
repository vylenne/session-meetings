mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod services;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, get, web};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use rand::rngs::OsRng;
use sqlx::PgPool;

use crate::config::AppConfig;

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();
    let pool = db::create_pool(&config.database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // seed test user
    seed_test_user(&pool).await;

    let config_data = web::Data::new(config);
    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(config_data.clone())
            .app_data(pool_data.clone())
            .service(health)
            .service(
                web::scope("/api/auth")
                    .route("/register", web::post().to(handlers::auth::register))
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("/me", web::get().to(handlers::auth::me)),
            )
            .service(
                web::scope("/api/meetings")
                    .route("", web::post().to(handlers::meetings::create))
                    .route("", web::get().to(handlers::meetings::list))
                    .route("/room/{room_name}", web::get().to(handlers::meetings::get_by_room))
                    .route("/{id}", web::get().to(handlers::meetings::get_one)),
            )
            .service(
                web::scope("/api/invite")
                    .route("/{token}", web::get().to(handlers::invitations::validate_invite)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn seed_test_user(pool: &PgPool) {
    let users = [
        ("test@example.com", "testtest", "Test User"),
        ("test2@example.com", "testtest", "Test User 2"),
    ];

    for (email, password, name) in &users {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
                .bind(email)
                .fetch_one(pool)
                .await
                .unwrap_or(false);

        if exists {
            continue;
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();

        sqlx::query("INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3)")
            .bind(email)
            .bind(&password_hash)
            .bind(name)
            .execute(pool)
            .await
            .expect("Failed to seed test user");

        eprintln!("Seeded test user: {email} / {password}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_health() {
        let app = test::init_service(App::new().service(health)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
