use std::collections::HashMap;

use serde_derive::Serialize;

use crate::{
    models::WhoCan,
    utils::response::{ApiErrorType, ValidationError},
};

use super::enums::who_can_validator::validate_who_can_enum_array_field;

#[derive(Debug, Serialize)]
pub struct UpdateUserPrivacyPreferencesData {
    pub who_can_reply: Option<Option<Vec<WhoCan>>>,
    pub who_can_like: Option<Option<Vec<WhoCan>>>,
    pub who_can_mention_me: Option<Option<Vec<WhoCan>>>,
    pub who_can_watch_new_hesses: Option<Option<Vec<WhoCan>>>,
    pub who_can_watch_replies: Option<Option<Vec<WhoCan>>>,
    pub who_can_watch_follows: Option<Option<Vec<WhoCan>>>,
    pub who_can_watch_likes: Option<Option<Vec<WhoCan>>>,
}

impl TryFrom<HashMap<String, serde_json::Value>> for UpdateUserPrivacyPreferencesData {
    type Error = ApiErrorType;

    fn try_from(value: HashMap<String, serde_json::Value>) -> Result<Self, Self::Error> {
        let mut errors = Vec::<ValidationError>::new();

        let who_can_reply = validate_who_can_enum_array_field(
            &value.get("whoCanReply"),
            "whoCanReply",
            &mut errors,
            true,
            true,
        );

        let who_can_like = validate_who_can_enum_array_field(
            &value.get("whoCanLike"),
            "whoCanLike",
            &mut errors,
            true,
            true,
        );

        let who_can_mention_me = validate_who_can_enum_array_field(
            &value.get("whoCanMentionMe"),
            "whoCanMentionMe",
            &mut errors,
            true,
            true,
        );

        let who_can_watch_new_hesses = validate_who_can_enum_array_field(
            &value.get("whoCanWatchNewHesses"),
            "whoCanWatchNewHesses",
            &mut errors,
            true,
            true,
        );

        let who_can_watch_replies = validate_who_can_enum_array_field(
            &value.get("whoCanWatchReplies"),
            "whoCanWatchReplies",
            &mut errors,
            true,
            true,
        );

        let who_can_watch_follows = validate_who_can_enum_array_field(
            &value.get("whoCanWatchFollows"),
            "whoCanWatchFollows",
            &mut errors,
            true,
            true,
        );

        let who_can_watch_likes = validate_who_can_enum_array_field(
            &value.get("whoCanWatchLikes"),
            "whoCanWatchLikes",
            &mut errors,
            true,
            true,
        );

        if errors.is_empty() {
            Ok(UpdateUserPrivacyPreferencesData {
                who_can_reply,
                who_can_like,
                who_can_mention_me,
                who_can_watch_new_hesses,
                who_can_watch_replies,
                who_can_watch_follows,
                who_can_watch_likes,
            })
        } else {
            Err(ApiErrorType::BodyValidationErrors(errors))
        }
    }
}
