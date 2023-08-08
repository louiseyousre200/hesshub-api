use std::collections::HashMap;

use crate::{
    models::WhoCan,
    utils::{
        response::ValidationError,
        validator::{validate_array_field, validate_enum_field},
    },
};

/// Validates an enum field based on allowed enum values.
///
/// This function is responsible for validating an enum field based on the given
/// `serde_json::Value` reference, the field's name, a mutable vector to collect
/// validation errors, and a boolean flag indicating whether the field is optional.
///
/// The `who_can_values` HashMap should contain allowed enum values as keys and their
/// corresponding string representations as values.
///
/// If the validation succeeds and the field's value matches an allowed enum variant,
/// it returns the corresponding `WhoCan` enum value. If the validation fails and the
/// field is not optional, a validation error is added to the errors vector.
///
/// # Parameters
///
/// - `value`: An `Option<&serde_json::Value>` representing the value of the enum field.
/// - `name`: The name of the enum field, used for error reporting.
/// - `errors`: A mutable reference to a vector of `ValidationError` to collect validation errors.
/// - `optional`: A boolean flag indicating whether the field is optional.
///
/// # Returns
///
/// - If the validation succeeds, returns an `Option<WhoCan>` with the corresponding enum value.
/// - If the validation fails and the field is not optional, returns `None`.
///
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

/// Validates an array of enum values based on a set of allowed enum values.
///
/// This function validates an array of enum values using the provided parameters:
/// - `value`: An optional reference to a `serde_json::Value` representing the array of enum values.
/// - `name`: The name of the array field, used for error reporting.
/// - `errors`: A mutable reference to a vector of `ValidationError` instances, used to store validation errors.
/// - `optional`: A boolean flag indicating if the array is optional. If `true`, the array's absence is not considered an error.
/// - `nullable`: A boolean flag indicating if the array can be nullable. If `true`, a `null` array is considered valid.
///
/// The function returns an `Option<Option<Vec<WhoCan>>>` indicating the validation result:
/// - If validation succeeds and the array is not `null`, it returns `Some(Some(Vec<WhoCan>))` with the validated enum values.
/// - If validation succeeds but the array is `null` (and nullable), it returns `Some(None)` to indicate no enum values.
/// - If validation fails or the array is optional and absent, it returns `None` and records validation errors.
///
/// Each individual enum value is validated using the `validate_who_can_enum_field` function.
///
/// # Parameters
///
/// - `value`: An optional reference to a `serde_json::Value` representing the array of enum values.
/// - `name`: The name of the array field, used for error reporting.
/// - `errors`: A mutable reference to a vector of `ValidationError` instances, used to collect validation errors.
/// - `optional`: A boolean flag indicating if the array is optional.
/// - `nullable`: A boolean flag indicating if the array can be nullable.
///
/// # Returns
///
/// - If validation succeeds and the array is not `null`, returns `Some(Some(Vec<WhoCan>))`.
/// - If validation succeeds but the array is `null` (and nullable), returns `Some(None)` to indicate no enum values.
/// - If validation fails or the array is optional and absent, returns `None` and adds validation errors.
///
/// The individual enum value validation relies on the `validate_who_can_enum_field` function.
///
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
