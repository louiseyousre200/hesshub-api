use std::collections::HashMap;

use crate::{
    models::UserRole,
    utils::{response::ValidationError, validator::validate_enum_field},
};

pub fn validate_user_role_enum_field(
    value: &Option<&serde_json::Value>,
    name: &str,
    errors: &mut Vec<ValidationError>,
    optional: bool,
) -> Option<UserRole> {
    let user_role_values: HashMap<UserRole, String> = [
        (UserRole::User, "USER".to_string()),
        (UserRole::Manager, "MANAGER".to_string()),
        (UserRole::Root, "ROOT".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    validate_enum_field(value, name, &user_role_values, errors, optional)
}
