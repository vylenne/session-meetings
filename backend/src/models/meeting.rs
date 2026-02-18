use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Meeting {
    pub id: Uuid,
    pub room_name: String,
    pub title: Option<String>,
    pub creator_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMeetingRequest {
    pub title: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MeetingResponse {
    pub id: Uuid,
    pub room_name: String,
    pub title: Option<String>,
    pub jitsi_jwt: String,
    pub invite_url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct MeetingListItem {
    pub id: Uuid,
    pub room_name: String,
    pub title: Option<String>,
    pub invite_url: String,
    pub created_at: DateTime<Utc>,
}
