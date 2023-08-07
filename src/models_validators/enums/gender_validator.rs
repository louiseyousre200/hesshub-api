use std::collections::HashMap;

use crate::{
    models::Gender,
    utils::{response::ValidationError, validator::validate_enum_field},
};

pub fn validate_gender_enum_field(
    value: &Option<&serde_json::Value>,
    name: &str,
    errors: &mut Vec<ValidationError>,
    optional: bool,
) -> Option<Gender> {
    let gender_values: HashMap<Gender, String> = [
        (Gender::Male, "MALE".to_string()),
        (Gender::Female, "FEMALE".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    validate_enum_field(value, name, &gender_values, errors, optional)
}
