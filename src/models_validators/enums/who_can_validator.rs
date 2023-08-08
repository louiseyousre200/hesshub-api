use std::collections::HashMap;

use crate::{
    models::WhoCan,
    utils::{response::ValidationError, validator::validate_enum_field},
};

pub fn validate_who_can_enum_field(
    value: &Option<&serde_json::Value>,
    name: &str,
    errors: &mut Vec<ValidationError>,
    optional: bool,
) -> Option<WhoCan> {
    let who_can_values: HashMap<WhoCan, String> = [
        (WhoCan::Followed, "FOLLOWED".to_string()),
        (WhoCan::Followers, "FOLLOWERS".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    validate_enum_field(value, name, &who_can_values, errors, optional)
}
