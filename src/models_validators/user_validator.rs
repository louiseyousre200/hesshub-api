use std::collections::HashMap;

use serde_derive::Serialize;

use crate::{
    models::{Gender, UserRole},
    utils::{
        response::{ApiErrorType, ValidationError},
        validator::{
            validate_boolean_field, validate_email_field, validate_string_field, FieldLength,
        },
    },
};

use super::enums::{
    gender_validator::validate_gender_enum_field,
    user_role_validator::validate_user_role_enum_field,
};

#[derive(Debug, Serialize)]
pub struct InsertUserData {
    pub name: String,
    pub gender: Gender,
    pub role: UserRole,
    pub bio: Option<String>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub activated: bool,
    pub verified: bool,
}

impl TryFrom<HashMap<String, serde_json::Value>> for InsertUserData {
    type Error = ApiErrorType;

    fn try_from(value: HashMap<String, serde_json::Value>) -> Result<Self, Self::Error> {
        let mut errors = Vec::<ValidationError>::new();

        let name = validate_string_field(
            &value.get("name"),
            "name",
            FieldLength {
                min: None,
                max: Some(255),
            },
            &mut errors,
            false,
        );

        let email = validate_email_field(
            &value.get("email"),
            "email",
            FieldLength {
                min: None,
                max: Some(255),
            },
            &mut errors,
            false,
        );

        let bio = validate_string_field(
            &value.get("bio"),
            "bio",
            FieldLength {
                min: None,
                max: Some(255),
            },
            &mut errors,
            true,
        );

        let username = validate_string_field(
            &value.get("username"),
            "username",
            FieldLength {
                min: None,
                max: Some(50),
            },
            &mut errors,
            false,
        );

        let password = validate_string_field(
            &value.get("password"),
            "password",
            FieldLength {
                min: None,
                max: Some(50),
            },
            &mut errors,
            false,
        );

        let verified =
            validate_boolean_field(&value.get("verified"), "verified", &mut errors, false);

        let activated =
            validate_boolean_field(&value.get("activated"), "activated", &mut errors, false);

        let gender = validate_gender_enum_field(&value.get("gender"), "gender", &mut errors, false);

        let user_role =
            validate_user_role_enum_field(&value.get("userRole"), "userRole", &mut errors, false);

        if errors.is_empty() {
            Ok(InsertUserData {
                name: name.unwrap(),
                email: email.unwrap(),
                username: username.unwrap(),
                password: password.unwrap(),
                gender: gender.unwrap(),
                bio,
                role: user_role.unwrap(),
                verified: verified.unwrap(),
                activated: activated.unwrap(),
            })
        } else {
            Err(ApiErrorType::BodyValidationErrors(errors))
        }
    }
}
