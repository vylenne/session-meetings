use actix_web::{HttpResponse, web};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;
use sqlx::PgPool;

use crate::config::AppConfig;
use crate::errors::ApiError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::{AuthResponse, LoginRequest, RegisterRequest, User, UserProfile};
use crate::services::jwt;

pub async fn register(
    pool: web::Data<PgPool>,
    config: web::Data<AppConfig>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ApiError> {
    if body.email.is_empty() || body.password.is_empty() || body.name.is_empty() {
        return Err(ApiError::BadRequest("All fields are required".into()));
    }

    if body.password.len() < 8 {
        return Err(ApiError::BadRequest("Password must be at least 8 characters".into()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .to_string();

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&body.email)
    .bind(&password_hash)
    .bind(&body.name)
    .fetch_one(pool.get_ref())
    .await?;

    let token = jwt::create_app_token(user.id, config.get_ref())?;

    Ok(HttpResponse::Created().json(AuthResponse { token }))
}

pub async fn login(
    pool: web::Data<PgPool>,
    config: web::Data<AppConfig>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&body.email)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiError::Unauthorized("Invalid email or password".into()))?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Unauthorized("Invalid email or password".into()))?;

    let token = jwt::create_app_token(user.id, config.get_ref())?;

    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}

pub async fn me(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(auth.user_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(UserProfile::from(user)))
}
