use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::MediaType;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HessMedia {
    pub id: Uuid,
    pub hess_id: Uuid,
    pub media_type: MediaType,
    pub media_url: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
