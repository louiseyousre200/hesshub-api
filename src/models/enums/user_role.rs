use serde_derive::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "USER_ROLE", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    User,
    Manager,
    Root,
}
