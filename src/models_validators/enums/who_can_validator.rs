use std::collections::HashMap;

use crate::{
    models::WhoCan,
    utils::{
        response::ValidationError,
        validator::{validate_array_field, validate_enum_field},
    },
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

pub fn validate_who_can_enum_array_field(
    value: &Option<&serde_json::Value>,
    name: &str,
    errors: &mut Vec<ValidationError>,
    optional: bool,
    nullable: bool,
) -> Option<Option<Vec<WhoCan>>> {
    let who_can = validate_array_field(value, name, errors, optional, nullable);

    match who_can {
        Some(Some(value)) => {
            let mut res = value
                .into_iter()
                .map(|ref v| validate_who_can_enum_field(&Some(v), name, errors, false));

            if res.any(|v| v.is_none()) {
                None
            } else {
                Some(Some(res.map(Option::unwrap).collect()))
            }
        }
        Some(None) => Some(None),
        None => None,
    }
}
