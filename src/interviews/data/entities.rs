use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Interview {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub youtube_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
