use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowRequest {
    pub id: Uuid,
    pub requester_id: Uuid,
    pub requested_id: Uuid,
    pub watch_new_hesses: bool,
    pub watch_replies: bool,
    pub watch_follows: bool,
    pub watch_likes: bool,
    pub status: FollowRequestStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "FOLLOW_REQUEST_STATUS")]
pub enum FollowRequestStatus {
    Pending,
    Approved,
    Rejected,
}
