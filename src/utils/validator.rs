use regex::Regex;
use serde_derive::Serialize;
use std::collections::HashMap;

use super::response::ValidationError;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FieldType {
    Array,
    Bool,
    Null,
    Number,
    Object,
    String,
}

/// Represents length constraints for a field's value.
///
/// This struct defines the minimum and/or maximum lengths that a field's value is allowed to have.
///
/// # Fields
///
/// - `min`: An optional `usize` specifying the minimum allowed length for the field's value.
///   If `Some`, the field's value must have a length greater than or equal to `min`.
///
/// - `max`: An optional `usize` specifying the maximum allowed length for the field's value.
///   If `Some`, the field's value must have a length less than or equal to `max`.
#[derive(Debug, Serialize)]
pub struct FieldLength {
    /// An optional `usize` specifying the minimum allowed length for the field's value.
    ///
    /// If `Some`, the field's value must have a length greater than or equal to `min`.
    pub min: Option<usize>,

    /// An optional `usize` specifying the maximum allowed length for the field's value.
    ///
    /// If `Some`, the field's value must have a length less than or equal to `max`.
    pub max: Option<usize>,
}

pub fn validate_email_field(
    value: &Option<&serde_json::Value>,
    name: &str,
    field_length: FieldLength,
    errors: &mut Vec<ValidationError>,
    optional: bool,
) -> Option<String> {
    let email_regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();

    let email_value = validate_string_field(value, name, field_length, errors, optional);

    if let Some(email) = &email_value {
        if !email_regex.is_match(email.as_str()) {
            errors.push(ValidationError::InvalidEmailFormat {
                passed_value: email.to_string(),
            });
        }
    }

    email_value
}

pub fn validate_telephone_number_field(
    value: &Option<&serde_json::Value>,
    name: &str,
    field_length: FieldLength,
    errors: &mut Vec<ValidationError>,
    optional: bool,
) -> Option<String> {
    let phone_number_regex =
        Regex::new(r"^[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}$").unwrap();

    let telephone_number = validate_string_field(value, name, field_length, errors, optional);

    if let Some(telephone_number) = &telephone_number {
        if !phone_number_regex.is_match(telephone_number.as_str()) {
            errors.push(ValidationError::InvalidTelephoneFormat {
                passed_value: telephone_number.to_string(),
            });
        }
    }

    telephone_number
}

pub fn validate_enum_field<E>(
    value: &Option<&serde_json::Value>,
    name: &str,
    expected_values: &HashMap<E, String>,
    errors: &mut Vec<ValidationError>,
    optional: bool,
) -> Option<E>
where
    E: PartialEq + Clone,
{
    match value {
        Some(serde_json::Value::String(value)) => {
            for (enum_value, str_value) in expected_values {
                if str_value == value {
                    return Some(enum_value.clone());
                }
            }
            errors.push(ValidationError::IncorrectEnumValue {
                field_name: name.to_string(),
                passed_value: value.to_string(),
                expected_values: expected_values.values().cloned().collect(),
            });
            None
        }
        None => {
            if !optional {
                errors.push(ValidationError::RequiredFieldMissing {
                    field_name: name.to_string(),
                });
            }
            None
        }
        _ => {
            errors.push(ValidationError::InvalidFieldDataType {
                field_name: name.to_string(),
                expected_type: FieldType::String,
            });
            None
        }
    }
}

pub fn validate_boolean_field(
    value: &Option<&serde_json::Value>,
    name: &str,
    errors: &mut Vec<ValidationError>,
    optional: bool,
) -> Option<bool> {
    match value {
        Some(serde_json::Value::Bool(value)) => Some(value.to_owned()),
        None => {
            if !optional {
                errors.push(ValidationError::RequiredFieldMissing {
                    field_name: name.to_string(),
                });
            }
            None
        }
        _ => {
            errors.push(ValidationError::InvalidFieldDataType {
                field_name: name.to_string(),
                expected_type: FieldType::Bool,
            });
            None
        }
    }
}

/// Validates a JSON field that is expected to contain an array.
///
/// This function validates a specific field in a JSON-like data structure. It checks
/// whether the field is an array and performs additional validations based on the
/// provided parameters. It supports scenarios where the field is optional or nullable.
///
/// # Parameters
///
/// - `value`: A reference to an `Option` containing a reference to a `serde_json::Value`.
///   This represents the value of the field that needs to be validated.
///
/// - `name`: A string slice (`&str`) representing the name of the field being validated.
///
/// - `errors`: A mutable reference to a vector (`&mut Vec<ValidationError>`) that will
///   store any validation errors encountered during the validation process.
///
/// - `optional`: A boolean indicating whether the field is optional (if `true`) or
///   required (if `false`).
///
/// - `nullable`: A boolean indicating whether the field can be `null` (if `true`) or
///   not (if `false`).
///
/// # Returns
///
/// This function returns a nested `Option<Vec<serde_json::Value>>` to represent different
/// outcomes:
///
/// - `Some(Some(...))`: If the validation is successful and the field contains an array,
///   it returns `Some` wrapping `Some` with a cloned vector of `serde_json::Value` elements.
///
/// - `Some(None)`: If the validation is successful but the field is `null` (and `nullable` is `true`),
///   it returns `Some` wrapping `None`.
///
/// - `None`: If the validation encounters an error or the field is missing. If the field is optional,
///   it may return `None` to indicate that the field is not present without indicating an error.
///
pub fn validate_array_field(
    value: &Option<&serde_json::Value>,
    name: &str,
    errors: &mut Vec<ValidationError>,
    optional: bool,
    nullable: bool,
) -> Option<Option<Vec<serde_json::Value>>> {
    match value {
        Some(serde_json::Value::Array(value)) => Some(Some(value.to_owned())),
        Some(serde_json::Value::Null) => {
            if nullable {
                Some(None)
            } else {
                errors.push(ValidationError::InvalidFieldDataType {
                    field_name: name.to_string(),
                    expected_type: FieldType::Array,
                });
                None
            }
        }
        None => {
            if !optional {
                errors.push(ValidationError::RequiredFieldMissing {
                    field_name: name.to_string(),
                });
            }
            None
        }
        _ => {
            errors.push(ValidationError::InvalidFieldDataType {
                field_name: name.to_string(),
                expected_type: FieldType::Bool,
            });
            None
        }
    }
}

/// Validates a JSON field that is expected to contain a string.
///
/// This function validates a specific field in a JSON-like data structure. It checks whether
/// the field is a string and performs additional validations based on the provided parameters.
/// It supports scenarios where the field is optional or required, and enforces length constraints.
///
/// # Parameters
///
/// - `value`: A reference to an `Option` containing a reference to a `serde_json::Value`.
///   This represents the value of the field that needs to be validated.
///
/// - `name`: A string slice (`&str`) representing the name of the field being validated.
///
/// - `field_length`: A `FieldLength` struct defining the minimum and/or maximum lengths
///   for the string field's value.
///
/// - `errors`: A mutable reference to a vector (`&mut Vec<ValidationError>`) that will
///   store any validation errors encountered during the validation process.
///
/// - `optional`: A boolean indicating whether the field is optional (if `true`) or
///   required (if `false`).
///
/// # Returns
///
/// This function returns a nested `Option<String>` to represent different outcomes:
///
/// - `Some(value)`: If the validation is successful and the field contains a valid string
///   within the specified length constraints, it returns `Some` wrapping the validated string.
///
/// - `None`: If the validation encounters an error or the field is missing. If the field
///   is optional, it may return `None` to indicate that the field is not present without
///   indicating an error.
///
/// # Errors
///
/// If validation fails, the function appends one of the following `ValidationError` variants
/// to the `errors` vector:
///
/// - `InvalidFieldContentLength`: If the string's length is outside the specified range.
///
/// - `RequiredFieldMissing`: If a required field is missing.
///
/// - `InvalidFieldDataType`: If the field does not contain a string value.
///
pub fn validate_string_field(
    value: &Option<&serde_json::Value>,
    name: &str,
    field_length: FieldLength,
    errors: &mut Vec<ValidationError>,
    optional: bool,
) -> Option<String> {
    match value {
        Some(serde_json::Value::String(value)) => {
            let len = value.len();
            if let Some(min) = field_length.min {
                if len < min {
                    errors.push(ValidationError::InvalidFieldContentLength {
                        field_name: name.to_string(),
                        passed_length: len,
                        expected_length: FieldLength {
                            min: Some(min),
                            max: field_length.max,
                        },
                    });
                    None
                } else if let Some(max) = field_length.max {
                    if len > max {
                        errors.push(ValidationError::InvalidFieldContentLength {
                            field_name: name.to_string(),
                            passed_length: len,
                            expected_length: FieldLength {
                                min: field_length.min,
                                max: Some(max),
                            },
                        });
                        None
                    } else {
                        Some(value.to_string())
                    }
                } else {
                    Some(value.to_string())
                }
            } else if let Some(max) = field_length.max {
                if len > max {
                    errors.push(ValidationError::InvalidFieldContentLength {
                        field_name: name.to_string(),
                        passed_length: len,
                        expected_length: FieldLength {
                            min: field_length.min,
                            max: Some(max),
                        },
                    });
                    None
                } else {
                    Some(value.to_string())
                }
            } else {
                Some(value.to_string())
            }
        }
        None => {
            if !optional {
                errors.push(ValidationError::RequiredFieldMissing {
                    field_name: name.to_string(),
                });
            }
            None
        }
        _ => {
            errors.push(ValidationError::InvalidFieldDataType {
                field_name: name.to_string(),
                expected_type: FieldType::String,
            });
            None
        }
    }
}
