use actix_web::{FromRequest, HttpRequest, dev::Payload};
use std::future::{Ready, ready};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::errors::ApiError;
use crate::services::jwt;

pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let result = extract_user(req);
        ready(result)
    }
}

fn extract_user(req: &HttpRequest) -> Result<AuthenticatedUser, ApiError> {
    let config = req
        .app_data::<actix_web::web::Data<AppConfig>>()
        .ok_or_else(|| ApiError::Internal("AppConfig not found".into()))?;

    let header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ApiError::Unauthorized("Missing Authorization header".into()))?;

    let token = header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ApiError::Unauthorized("Invalid Authorization format".into()))?;

    let claims = jwt::verify_app_token(token, config)?;

    Ok(AuthenticatedUser {
        user_id: claims.sub,
    })
}
