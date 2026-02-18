use chrono::{DateTime, Utc};
use serde::Serialize;
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
