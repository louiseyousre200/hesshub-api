use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    models::BlockedUser,
    utils::response::{ApiErrorType, ApiResource},
};

/// Inserts a new blocked user entry into the database.
///
/// This function is responsible for inserting a new blocked user entry into the database,
/// indicating that one user (the blocker) has blocked another user (the blocked).
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `blocker_id` - UUID of the user who is blocking.
/// * `blocked_id` - UUID of the user who is being blocked.
///
/// # Returns
///
/// Returns a `Result` containing the inserted `BlockedUser` if successful.
/// If any error occurs during database insertion, returns an `ApiErrorType::InternalServerError` error.
///
pub async fn insert_blocked_user(
    pool: Pool<Postgres>,
    blocker_id: Uuid,
    blocked_id: Uuid,
) -> Result<BlockedUser, ApiErrorType> {
    let query_result = sqlx::query_as!(
        BlockedUser,
        r#"INSERT INTO blocked_users (blocker_id, blocked_id) VALUES ($1, $2)
        RETURNING id, blocker_id, blocked_id, created_at, deleted_at
        "#,
        blocker_id,
        blocked_id
    )
    .fetch_one(&pool)
    .await;

    match query_result {
        Ok(blocked_user) => Ok(blocked_user),
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}

/// Softly deletes a blocked user entry from the database.
///
/// This function performs a "soft" delete by setting the `deleted_at` timestamp for the blocked user entry
/// with the provided UUID, indicating that the entry has been logically deleted. The blocked user's data remains
/// in the database, but it is considered inactive. This approach helps maintain data integrity and allows for potential
/// data recovery if needed.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `id` - The UUID of the blocked user entry to delete.
///
/// # Returns
///
/// Returns a `Result<(), ApiErrorType>` indicating success or an `ApiErrorType::InternalServerError` error
/// if any error occurs during database deletion.
/// If no blocked user entry with the provided ID is found for deletion or if the entry is already soft deleted,
/// returns an `ApiErrorType::ResourceNotFound` error.
///
pub async fn delete_blocked_user(pool: Pool<Postgres>, id: Uuid) -> Result<(), ApiErrorType> {
    let current_date = Utc::now();

    let query_result = sqlx::query!(
        "UPDATE followers SET deleted_at = $1 WHERE id = $2 AND deleted_at IS NULL",
        current_date,
        id,
    )
    .execute(&pool)
    .await;

    match query_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Err(ApiErrorType::ResourceNotFound(ApiResource::BlockedUsers))
            } else {
                Ok(())
            }
        }
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}
