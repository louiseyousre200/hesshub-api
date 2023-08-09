use std::num::IntErrorKind;

use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use warp::hyper::StatusCode;

use super::validator::{FieldLength, FieldType};

#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiResource {
    Users,
    PasswordResetToken,
    UserConfirmationToken,
    Followers,
    BlockerUsers,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidationError {
    InvalidJsonBody,
    RequiredFieldMissing {
        field_name: String,
    },
    InvalidFieldDataType {
        field_name: String,
        expected_type: FieldType,
    },
    InvalidFieldContentLength {
        field_name: String,
        passed_length: usize,
        expected_length: FieldLength,
    },
    IncorrectEnumValue {
        field_name: String,
        passed_value: String,
        expected_values: Vec<String>,
    },
    InvalidEmailFormat {
        passed_value: String,
    },
    InvalidTelephoneFormat {
        passed_value: String,
    },
}

#[derive(Debug)]
pub enum ApiErrorType {
    // Not Found
    ResourceNotFound(ApiResource),
    RouteNotFound,

    // Authentication & Authorization
    Unauthorized,
    NotLoggedIn,
    InvalidJwtToken,
    InvalidCredentials,
    PasswordResetTokenExpired,
    UserConfirmationTokenExpired,

    // Unidentifiable
    InternalServerError,

    // Generic Validation
    BodyValidationErrors(Vec<ValidationError>),

    // Query Validation
    InvalidSortingQuerySyntax,
    NonExistantSortingQueryField(String),
    InvalidPaginationPageQueryField(IntErrorKind),
    InvalidPaginationSizeQueryField(IntErrorKind),

    // Parameters Validation
    InvalidIdParam(ApiResource),

    // Already existing
    AlreadyExists(ApiResource),

    // File Uploading
    NotAnImage,
    UnnamedMultipartFile,
    EmptyFile,
    NoImage,
}

fn int_error_kind_to_str(kind: &IntErrorKind) -> Option<String> {
    match kind {
        IntErrorKind::Empty => Some("EMPTY".to_string()),
        IntErrorKind::InvalidDigit => Some("INVALID_DIGIT".to_string()),
        IntErrorKind::PosOverflow => Some("POS_OVERFLOW".to_string()),
        IntErrorKind::NegOverflow => Some("NEG_OVERFLOW".to_string()),
        IntErrorKind::Zero => Some("ZERO".to_string()),
        _ => None,
    }
}

impl ApiErrorType {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiErrorType::Unauthorized => StatusCode::FORBIDDEN,
            ApiErrorType::UserConfirmationTokenExpired
            | ApiErrorType::PasswordResetTokenExpired
            | ApiErrorType::InvalidCredentials
            | ApiErrorType::NotLoggedIn
            | ApiErrorType::InvalidJwtToken => StatusCode::UNAUTHORIZED,
            ApiErrorType::RouteNotFound | ApiErrorType::ResourceNotFound(_) => {
                StatusCode::NOT_FOUND
            }
            ApiErrorType::NoImage
            | ApiErrorType::EmptyFile
            | ApiErrorType::NotAnImage
            | ApiErrorType::UnnamedMultipartFile
            | ApiErrorType::InvalidIdParam(_)
            | ApiErrorType::BodyValidationErrors(_)
            | ApiErrorType::InvalidPaginationPageQueryField(_)
            | ApiErrorType::InvalidPaginationSizeQueryField(_)
            | ApiErrorType::InvalidSortingQuerySyntax
            | ApiErrorType::NonExistantSortingQueryField(_) => StatusCode::BAD_REQUEST,
            ApiErrorType::AlreadyExists(_) => StatusCode::CONFLICT,
            ApiErrorType::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn details(&self) -> Option<serde_json::Value> {
        match self {
            ApiErrorType::UserConfirmationTokenExpired
            | ApiErrorType::PasswordResetTokenExpired
            | ApiErrorType::NoImage
            | ApiErrorType::UnnamedMultipartFile
            | ApiErrorType::EmptyFile
            | ApiErrorType::NotAnImage
            | ApiErrorType::RouteNotFound
            | ApiErrorType::Unauthorized
            | ApiErrorType::NotLoggedIn
            | ApiErrorType::InvalidJwtToken
            | ApiErrorType::InvalidCredentials
            | ApiErrorType::InternalServerError
            | ApiErrorType::InvalidSortingQuerySyntax => None,
            ApiErrorType::InvalidIdParam(details) => Some(json!(details)),
            ApiErrorType::ResourceNotFound(details) => Some(json!(details)),
            ApiErrorType::BodyValidationErrors(details) => Some(json!(details)),
            ApiErrorType::NonExistantSortingQueryField(details) => Some(json!(details)),
            ApiErrorType::InvalidPaginationPageQueryField(details) => {
                Some(json!(int_error_kind_to_str(details)))
            }
            ApiErrorType::InvalidPaginationSizeQueryField(details) => {
                Some(json!(int_error_kind_to_str(details)))
            }
            ApiErrorType::AlreadyExists(details) => Some(json!(details)),
        }
    }

    pub fn code<'a>(&self) -> &'a str {
        match self {
            ApiErrorType::ResourceNotFound(_) => "RESOURCE_NOT_FOUND",
            ApiErrorType::RouteNotFound => "ROUTE_NOT_FOUND",
            ApiErrorType::Unauthorized => "UNAUTHORIZED",

            ApiErrorType::BodyValidationErrors(_) => "BODY_VALIDATION_ERRORS",
            ApiErrorType::InvalidSortingQuerySyntax => "INVALID_SORTING_QUERY_SYNTAX",
            ApiErrorType::NonExistantSortingQueryField(_) => "INVALID_SORTING_QUERY_FIELD",
            ApiErrorType::InvalidPaginationPageQueryField(_) => {
                "INVALID_PAGINATION_PAGE_QUERY_FIELD"
            }
            ApiErrorType::InvalidPaginationSizeQueryField(_) => {
                "INVALID_PAGINATION_SIZE_QUERY_FIELD"
            }

            ApiErrorType::NotLoggedIn => "NOT_LOGGED_IN",
            ApiErrorType::InvalidJwtToken => "INVALID_JWT_TOKEN",
            ApiErrorType::InvalidCredentials => "INVALID_CREDENTIALS",

            ApiErrorType::AlreadyExists(_) => "ALREADY_EXISTS",

            ApiErrorType::InternalServerError => "INTERNAL_SERVER_ERROR",
            ApiErrorType::InvalidIdParam(_) => "INVALID_ID_PARAM",
            ApiErrorType::UnnamedMultipartFile => "UNNAMED_MULTIPART_FILE",
            ApiErrorType::NotAnImage => "NOT_AN_IMAGE",
            ApiErrorType::EmptyFile => "EMPTY_FILE",
            ApiErrorType::NoImage => "NO_IMAGE",
            ApiErrorType::PasswordResetTokenExpired => "PASSWORD_RESET_TOKEN_EXPIRED",
            ApiErrorType::UserConfirmationTokenExpired => "USER_CONFIRMATION_TOKEN_EXPIRED",
        }
    }
}

impl warp::reject::Reject for ApiErrorType {}

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse<T: serde::Serialize> {
    pub code: String,
    pub details: T,
}
