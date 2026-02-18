use actix_web::{HttpResponse, web};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::errors::ApiError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::invitation::Invitation;
use crate::models::meeting::{CreateMeetingRequest, Meeting, MeetingListItem, MeetingResponse};
use crate::models::user::User;
use crate::services::{invite, jwt};

pub async fn create(
    pool: web::Data<PgPool>,
    config: web::Data<AppConfig>,
    auth: AuthenticatedUser,
    body: web::Json<CreateMeetingRequest>,
) -> Result<HttpResponse, ApiError> {
    let room_name = invite::generate_room_name();
    let invite_token = invite::generate_invite_token();

    let meeting = sqlx::query_as::<_, Meeting>(
        "INSERT INTO meetings (room_name, title, creator_id) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&room_name)
    .bind(&body.title)
    .bind(auth.user_id)
    .fetch_one(pool.get_ref())
    .await?;

    let _invitation = sqlx::query_as::<_, Invitation>(
        "INSERT INTO invitations (meeting_id, token, created_by, expires_at) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(meeting.id)
    .bind(&invite_token)
    .bind(auth.user_id)
    .bind(Utc::now() + Duration::days(7))
    .fetch_one(pool.get_ref())
    .await?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(auth.user_id)
        .fetch_one(pool.get_ref())
        .await?;

    let jitsi_jwt = jwt::create_jitsi_token(&room_name, &user.name, &user.email, config.get_ref())?;
    let invite_url = format!("/join/{invite_token}");

    Ok(HttpResponse::Created().json(MeetingResponse {
        id: meeting.id,
        room_name: meeting.room_name,
        title: meeting.title,
        jitsi_jwt,
        invite_url,
        created_at: meeting.created_at,
    }))
}

pub async fn list(
    pool: web::Data<PgPool>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    let rows = sqlx::query_as::<_, Meeting>(
        "SELECT * FROM meetings WHERE creator_id = $1 ORDER BY created_at DESC",
    )
    .bind(auth.user_id)
    .fetch_all(pool.get_ref())
    .await?;

    let meeting_ids: Vec<Uuid> = rows.iter().map(|m| m.id).collect();

    let invitations = sqlx::query_as::<_, Invitation>(
        "SELECT DISTINCT ON (meeting_id) * FROM invitations WHERE meeting_id = ANY($1) ORDER BY meeting_id, created_at DESC",
    )
    .bind(&meeting_ids)
    .fetch_all(pool.get_ref())
    .await?;

    let items: Vec<MeetingListItem> = rows
        .into_iter()
        .map(|m| {
            let invite_url = invitations
                .iter()
                .find(|inv| inv.meeting_id == m.id)
                .map(|inv| format!("/join/{}", inv.token))
                .unwrap_or_default();
            MeetingListItem {
                id: m.id,
                room_name: m.room_name,
                title: m.title,
                invite_url,
                created_at: m.created_at,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(items))
}

pub async fn get_one(
    pool: web::Data<PgPool>,
    config: web::Data<AppConfig>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let meeting_id = path.into_inner();

    let meeting = sqlx::query_as::<_, Meeting>(
        "SELECT * FROM meetings WHERE id = $1 AND creator_id = $2",
    )
    .bind(meeting_id)
    .bind(auth.user_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound("Meeting not found".into()))?;

    let invitation = sqlx::query_as::<_, Invitation>(
        "SELECT * FROM invitations WHERE meeting_id = $1 ORDER BY created_at DESC LIMIT 1",
    )
    .bind(meeting.id)
    .fetch_optional(pool.get_ref())
    .await?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(auth.user_id)
        .fetch_one(pool.get_ref())
        .await?;

    let jitsi_jwt = jwt::create_jitsi_token(&meeting.room_name, &user.name, &user.email, config.get_ref())?;
    let invite_url = invitation
        .map(|inv| format!("/join/{}", inv.token))
        .unwrap_or_default();

    Ok(HttpResponse::Ok().json(MeetingResponse {
        id: meeting.id,
        room_name: meeting.room_name,
        title: meeting.title,
        jitsi_jwt,
        invite_url,
        created_at: meeting.created_at,
    }))
}
