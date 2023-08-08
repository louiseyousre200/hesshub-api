use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    models::PasswordResetToken,
    utils::response::{ApiErrorType, ApiResource},
};

/// Retrieves a valid password reset token by its ID.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `id` - The UUID of the password reset token to retrieve.
///
/// # Returns
///
/// Returns a `Result` containing the retrieved `PasswordResetToken` if successful.
/// If the token is expired, returns an `ApiErrorType::PasswordResetTokenExpired` error.
/// If no token is found, returns an `ApiErrorType::ResourceNotFound` error.
/// If any other error occurs during database query, returns an `ApiErrorType::InternalServerError` error.
///
pub async fn get_valid_password_reset_token_by_id(
    pool: Pool<Postgres>,
    id: Uuid,
) -> Result<PasswordResetToken, ApiErrorType> {
    let query_result = sqlx::query_as!(
        PasswordResetToken,
        r#"SELECT * FROM password_reset_tokens WHERE id = $1"#,
        id
    )
    .fetch_optional(&pool)
    .await;

    match query_result {
        Ok(Some(password_reset_token)) if password_reset_token.expire_at < Utc::now() => {
            Ok(password_reset_token)
        }
        Ok(Some(_)) => Err(ApiErrorType::PasswordResetTokenExpired),
        Ok(None) => Err(ApiErrorType::ResourceNotFound(
            ApiResource::PasswordResetToken,
        )),
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}

/// Inserts a new password reset token into the database.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `user_id` - The UUID of the user associated with the token.
/// * `expire_at` - The expiration timestamp for the token.
///
/// # Returns
///
/// Returns a `Result` containing the inserted `PasswordResetToken` if successful.
/// If any error occurs during database insertion, returns an `ApiErrorType::InternalServerError` error.
///
pub async fn insert_password_reset_token(
    pool: Pool<Postgres>,
    user_id: Uuid,
    expire_at: DateTime<Utc>,
) -> Result<PasswordResetToken, ApiErrorType> {
    let query_result = sqlx::query_as!(
        PasswordResetToken,
        r#"
        INSERT INTO password_reset_tokens (user_id, expire_at) VALUES ($1, $2)
        RETURNING id,user_id, used, created_at, expire_at
        "#,
        user_id,
        expire_at
    )
    .fetch_one(&pool)
    .await;

    match query_result {
        Ok(password_reset_token) => Ok(password_reset_token),
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}

/// Deletes expired password reset tokens from the database.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
///
/// # Returns
///
/// Returns `Result<(), ApiErrorType>` indicating success or an `ApiErrorType::InternalServerError` error
/// if any error occurs during database deletion.
///
pub async fn delete_expired_password_reset_tokens(
    pool: Pool<Postgres>,
) -> Result<(), ApiErrorType> {
    let result = sqlx::query!(r#"DELETE FROM password_reset_tokens WHERE expire_at < now()"#,)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}
