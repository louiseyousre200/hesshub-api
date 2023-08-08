use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    models::UserConfirmationToken,
    utils::response::{ApiErrorType, ApiResource},
};

/// Retrieves a valid user confirmation token by its ID.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `id` - The UUID of the user confirmation token to retrieve.
///
/// # Returns
///
/// Returns a `Result` containing the retrieved `UserConfirmationToken` if successful.
/// If the token is expired, returns an `ApiErrorType::UserConfirmationTokenExpired` error.
/// If no token is found, returns an `ApiErrorType::ResourceNotFound` error.
/// If any other error occurs during the database query, returns an `ApiErrorType::InternalServerError` error.
///
pub async fn get_valid_user_confirmation_token_by_id(
    pool: Pool<Postgres>,
    id: Uuid,
) -> Result<UserConfirmationToken, ApiErrorType> {
    let query_result = sqlx::query_as!(
        UserConfirmationToken,
        r#"SELECT * FROM user_confirmation_tokens WHERE id = $1"#,
        id
    )
    .fetch_optional(&pool)
    .await;

    match query_result {
        Ok(Some(user_confirmation_token)) if user_confirmation_token.expire_at < Utc::now() => {
            Ok(user_confirmation_token)
        }
        Ok(Some(_)) => Err(ApiErrorType::UserConfirmationTokenExpired),
        Ok(None) => Err(ApiErrorType::ResourceNotFound(
            ApiResource::UserConfirmationToken,
        )),
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}

/// Inserts a new user confirmation token into the database.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `user_id` - The UUID of the user associated with the token.
/// * `expire_at` - The expiration timestamp for the token.
///
/// # Returns
///
/// Returns a `Result` containing the inserted `UserConfirmationToken` if successful.
/// If any error occurs during database insertion, returns an `ApiErrorType::InternalServerError` error.
///
pub async fn insert_user_confirmation_token(
    pool: Pool<Postgres>,
    user_id: Uuid,
    expire_at: DateTime<Utc>,
) -> Result<UserConfirmationToken, ApiErrorType> {
    let query_result = sqlx::query_as!(
        UserConfirmationToken,
        r#"
        INSERT INTO user_confirmation_tokens (user_id, expire_at) VALUES ($1, $2)
        RETURNING id,user_id, used, created_at, expire_at
        "#,
        user_id,
        expire_at
    )
    .fetch_one(&pool)
    .await;

    match query_result {
        Ok(user_confirmation_token) => Ok(user_confirmation_token),
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}

/// Deletes expired user confirmation tokens from the database.
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
pub async fn delete_expired_user_confirmation_tokens(
    pool: Pool<Postgres>,
) -> Result<(), ApiErrorType> {
    let result = sqlx::query!(r#"DELETE FROM user_confirmation_tokens WHERE expire_at < now()"#,)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}
