use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::AppConfig;

// --- App JWT (API authorization) ---

#[derive(Debug, Serialize, Deserialize)]
pub struct AppClaims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}

pub fn create_app_token(user_id: Uuid, config: &AppConfig) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = AppClaims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: (now + Duration::hours(24)).timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.app_jwt_secret.as_bytes()),
    )
}

pub fn verify_app_token(token: &str, config: &AppConfig) -> Result<AppClaims, jsonwebtoken::errors::Error> {
    let data = decode::<AppClaims>(
        token,
        &DecodingKey::from_secret(config.app_jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

// --- Jitsi JWT (room access) ---

#[derive(Debug, Serialize)]
pub struct JitsiClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub room: String,
    pub exp: usize,
    pub iat: usize,
    pub context: JitsiContext,
}

#[derive(Debug, Serialize)]
pub struct JitsiContext {
    pub user: JitsiUser,
}

#[derive(Debug, Serialize)]
pub struct JitsiUser {
    pub name: String,
    pub email: String,
}

pub fn create_jitsi_token(
    room_name: &str,
    user_name: &str,
    user_email: &str,
    config: &AppConfig,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = JitsiClaims {
        iss: config.jitsi_jwt_app_id.clone(),
        sub: config.jitsi_domain.clone(),
        aud: config.jitsi_jwt_app_id.clone(),
        room: room_name.to_string(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::minutes(15)).timestamp() as usize,
        context: JitsiContext {
            user: JitsiUser {
                name: user_name.to_string(),
                email: user_email.to_string(),
            },
        },
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jitsi_jwt_app_secret.as_bytes()),
    )
}
