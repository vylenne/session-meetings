use actix_web::{HttpRequest, HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;

use crate::config::AppConfig;
use crate::errors::ApiError;
use crate::models::invitation::{InviteRedirect, InviteResponse, Invitation};
use crate::models::meeting::Meeting;
use crate::models::user::User;
use crate::services::jwt;

pub async fn validate_invite(
    pool: web::Data<PgPool>,
    config: web::Data<AppConfig>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let token = path.into_inner();

    let invitation = sqlx::query_as::<_, Invitation>(
        "SELECT * FROM invitations WHERE token = $1",
    )
    .bind(&token)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound("Invalid invite link".into()))?;

    if invitation.expires_at < Utc::now() {
        return Err(ApiError::BadRequest("Invite link has expired".into()));
    }

    let meeting = sqlx::query_as::<_, Meeting>(
        "SELECT * FROM meetings WHERE id = $1",
    )
    .bind(invitation.meeting_id)
    .fetch_one(pool.get_ref())
    .await?;

    let user_id = try_extract_user(&req, &config);

    match user_id {
        Some(uid) => {
            let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(uid)
                .fetch_one(pool.get_ref())
                .await?;

            let jitsi_jwt = jwt::create_jitsi_token(
                &meeting.room_name,
                &user.name,
                &user.email,
                config.get_ref(),
            )?;

            Ok(HttpResponse::Ok().json(InviteResponse {
                room_name: meeting.room_name,
                jitsi_jwt,
            }))
        }
        None => {
            let redirect = format!("/login?redirect=/join/{token}");
            Ok(HttpResponse::Ok().json(InviteRedirect { redirect }))
        }
    }
}

fn try_extract_user(req: &HttpRequest, config: &AppConfig) -> Option<uuid::Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let claims = jwt::verify_app_token(token, config).ok()?;
    Some(claims.sub)
}
