use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::WhoCan;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPrivacyPreferences {
    pub id: Uuid,
    pub user_id: Uuid,
    pub is_private_profile: bool,
    pub who_can_reply: Option<Vec<WhoCan>>,
    pub who_can_like: Option<Vec<WhoCan>>,
    pub who_can_mention_me: Option<Vec<WhoCan>>,
    pub who_can_watch_new_hesses: Option<Vec<WhoCan>>,
    pub who_can_watch_replies: Option<Vec<WhoCan>>,
    pub who_can_watch_follows: Option<Vec<WhoCan>>,
    pub who_can_watch_likes: Option<Vec<WhoCan>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
