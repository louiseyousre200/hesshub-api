use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{Gender, UserRole};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub gender: Gender,
    pub role: UserRole,
    pub bio: Option<String>,
    pub email: String,
    pub user_profile_image_id: Option<Uuid>,
    pub username: String,
    pub password: String,
    pub activated: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<Uuid>,
}
