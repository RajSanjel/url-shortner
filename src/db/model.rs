use chrono;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ShortUrl {
    pub id: Uuid,
    pub url: String,
    pub custom_id: String,
    pub created_at: chrono::NaiveDateTime,
}
