use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
    Internal(String),
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad request: {msg}"),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {msg}"),
            ApiError::NotFound(msg) => write!(f, "Not found: {msg}"),
            ApiError::Conflict(msg) => write!(f, "Conflict: {msg}"),
            ApiError::Internal(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let body = ErrorBody {
            error: self.to_string(),
        };
        match self {
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(body),
            ApiError::Unauthorized(_) => HttpResponse::Unauthorized().json(body),
            ApiError::NotFound(_) => HttpResponse::NotFound().json(body),
            ApiError::Conflict(_) => HttpResponse::Conflict().json(body),
            ApiError::Internal(_) => HttpResponse::InternalServerError().json(body),
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ApiError::NotFound("Resource not found".into()),
            sqlx::Error::Database(ref db_err) if db_err.code().as_deref() == Some("23505") => {
                ApiError::Conflict("Resource already exists".into())
            }
            _ => ApiError::Internal(err.to_string()),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        ApiError::Unauthorized(err.to_string())
    }
}

impl From<argon2::password_hash::Error> for ApiError {
    fn from(err: argon2::password_hash::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}
