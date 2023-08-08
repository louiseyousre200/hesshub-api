use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    models::PasswordResetToken,
    utils::response::{ApiErrorType, ApiResource},
};

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
