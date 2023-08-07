use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserConfirmationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub used: bool,
    pub created_at: DateTime<Utc>,
    pub expire_at: DateTime<Utc>,
}
