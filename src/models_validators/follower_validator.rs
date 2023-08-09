use std::collections::HashMap;

use serde_derive::Serialize;

use crate::utils::{
    response::{ApiErrorType, ValidationError},
    validator::validate_boolean_field,
};

/// Represents the data used for updating or inserting follower preferences.
///
/// When inserting a new follower preference, if a field is set to `None`, it will default to the system default.
/// During updates, setting a field to `None` indicates that the corresponding preference will not be updated.
#[derive(Debug, Serialize)]
pub struct UpdateOrInsertFollowerData {
    /// Determines whether the follower should receive notifications for new hesses.
    pub watch_new_hesses: Option<bool>,

    /// Determines whether the follower should receive notifications for replies.
    pub watch_replies: Option<bool>,

    /// Determines whether the follower should receive notifications for new follows.
    pub watch_follows: Option<bool>,

    /// Determines whether the follower should receive notifications for likes.
    pub watch_likes: Option<bool>,
}

impl TryFrom<HashMap<String, serde_json::Value>> for UpdateOrInsertFollowerData {
    type Error = ApiErrorType;

    fn try_from(value: HashMap<String, serde_json::Value>) -> Result<Self, Self::Error> {
        let mut errors = Vec::<ValidationError>::new();

        let watch_follows = validate_boolean_field(
            &value.get("watchFollows"),
            "watchFollows",
            &mut errors,
            true,
        );

        let watch_likes =
            validate_boolean_field(&value.get("watchLikes"), "watchLikes", &mut errors, true);

        let watch_new_hesses = validate_boolean_field(
            &value.get("watchNewHesses"),
            "watchNewHesses",
            &mut errors,
            true,
        );

        let watch_replies = validate_boolean_field(
            &value.get("watchReplies"),
            "watchReplies",
            &mut errors,
            true,
        );

        if errors.is_empty() {
            Ok(UpdateOrInsertFollowerData {
                watch_follows,
                watch_likes,
                watch_new_hesses,
                watch_replies,
            })
        } else {
            Err(ApiErrorType::BodyValidationErrors(errors))
        }
    }
}
