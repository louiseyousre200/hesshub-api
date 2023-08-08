use std::collections::HashMap;

use crate::{
    models::Gender,
    utils::{response::ValidationError, validator::validate_enum_field},
};

/// Validates a gender enum field based on a set of allowed enum values.
///
/// This function validates a gender enum field using the provided parameters:
/// - `value`: A reference to an `Option<&serde_json::Value>` representing the gender enum value.
/// - `name`: The name of the gender field, used for error reporting.
/// - `errors`: A mutable reference to a vector of `ValidationError` instances, used to store validation errors.
/// - `optional`: A boolean flag indicating if the gender field is optional. If `true`, the absence of the gender field is not considered an error.
///
/// The function returns an `Option<Gender>` indicating the validation result:
/// - If validation succeeds and the gender enum value is valid, it returns `Some(Gender::ValidGender)`.
/// - If validation fails, it returns `None` and records validation errors.
/// - If the gender field is optional and absent, it returns `None`.
///
/// The function constructs a mapping of allowed gender enum values to their corresponding string representations.
/// The individual enum value validation is delegated to the `validate_enum_field` function.
///
/// # Parameters
///
/// - `value`: A reference to an `Option<&serde_json::Value>` representing the gender enum value.
/// - `name`: The name of the gender field, used for error reporting.
/// - `errors`: A mutable reference to a vector of `ValidationError` instances, used to collect validation errors.
/// - `optional`: A boolean flag indicating if the gender field is optional.
///
/// # Returns
///
/// - If validation succeeds and the gender enum value is valid and exists, returns `Some(Gender::ValidGender)`.
/// - If validation fails or the gender field isn't optional and absent, returns `None` and adds validation errors.
/// - If validation succeeds and the gender field is optional and absent, returns `None`.
///
/// The individual enum value validation relies on the `validate_enum_field` function.
///
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
