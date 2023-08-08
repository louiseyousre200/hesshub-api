use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Follower {
    pub id: Uuid,
    pub follower_id: Uuid,
    pub followed_id: Uuid,
    pub watch_new_hesses: bool,
    pub watch_replies: bool,
    pub watch_follows: bool,
    pub watch_likes: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
