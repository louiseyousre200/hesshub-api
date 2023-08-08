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

#[derive(Debug, Serialize)]
pub struct FieldLength {
    pub min: Option<usize>,
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
