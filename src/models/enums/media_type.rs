use serde_derive::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "MEDIA_TYPE", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    Audio,
    Video,
    Photo,
}
