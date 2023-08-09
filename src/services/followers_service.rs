use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    models::Follower,
    models_validators::follower_validator::UpdateOrInsertFollowerData,
    utils::response::{ApiErrorType, ApiResource},
};

/// Inserts a new follower relationship into the database.
///
/// This function is responsible for inserting a new follower relationship between two users
/// into the database. It allows one user (the follower) to follow another user (the followed).
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `insert_follower_data` - Data to insert for the new follower relationship.
/// * `follower_id` - UUID of the user who is the follower.
/// * `followed_id` - UUID of the user who is being followed.
///
/// # Returns
///
/// Returns a `Result` containing the inserted `Follower` if successful.
/// If any error occurs during database insertion, returns an `ApiErrorType::InternalServerError` error.
///
pub async fn insert_follower(
    pool: Pool<Postgres>,
    insert_follower_data: UpdateOrInsertFollowerData,
    follower_id: Uuid,
    followed_id: Uuid,
) -> Result<Follower, ApiErrorType> {
    let query_result = sqlx::query_as!(
        Follower,
        r#"INSERT INTO followers (
            follower_id,
            followed_id,
            watch_new_hesses,
            watch_replies,
            watch_follows,
            watch_likes
        ) VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            id,
            follower_id,
            followed_id,
            watch_new_hesses,
            watch_replies,
            watch_follows,
            watch_likes,
            created_at,
            updated_at,
            deleted_at
        "#,
        follower_id,
        followed_id,
        insert_follower_data.watch_new_hesses,
        insert_follower_data.watch_replies,
        insert_follower_data.watch_follows,
        insert_follower_data.watch_likes
    )
    .fetch_one(&pool)
    .await;

    match query_result {
        Ok(follower) => Ok(follower),
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}

/// Updates a follower relationship's information in the database.
///
/// This function allows updating specific fields of a follower relationship's information in the database.
/// It constructs a dynamic SQL query based on the provided fields to update and their values.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `update_follower_data` - Data containing the fields to update.
/// * `id` - The UUID of the follower relationship to update.
///
/// # Returns
///
/// Returns a `Result<(), ApiErrorType>` indicating success or an `ApiErrorType::InternalServerError` error
/// if any error occurs during database update.
/// If no follower relationship with the provided ID is found for updating or if no fields are provided for update,
/// returns an `ApiErrorType::ResourceNotFound` error.
///
pub async fn update_follower(
    pool: Pool<Postgres>,
    update_follower_data: UpdateOrInsertFollowerData,
    id: Uuid,
) -> Result<(), ApiErrorType> {
    let mut update_set = vec![];
    let mut counter = 1;

    if let Some(watch_follows) = &update_follower_data.watch_follows {
        update_set.push(format!("watch_follows = ${}", watch_follows));
        counter += 1;
    }

    if let Some(watch_likes) = &update_follower_data.watch_likes {
        update_set.push(format!(
            "watch_likes = ${}",
            serde_json::to_string(watch_likes).unwrap()
        ));
        counter += 1;
    }

    if let Some(watch_new_hesses) = &update_follower_data.watch_new_hesses {
        update_set.push(format!(
            "watch_new_hesses = ${}",
            serde_json::to_string(watch_new_hesses).unwrap()
        ));
        counter += 1;
    }

    if let Some(watch_replies) = &update_follower_data.watch_replies {
        update_set.push(format!("watch_replies = ${}", watch_replies));
        counter += 1;
    }

    if counter == 1 {
        return Ok(());
    }

    let query_string = format!(
        "UPDATE users SET {} WHERE id = ${} AND deleted_at IS NULL",
        update_set.join(", "),
        counter
    );

    let mut query = sqlx::query(query_string.as_str());

    if let Some(watch_follows) = &update_follower_data.watch_follows {
        query = query.bind(watch_follows);
    }

    if let Some(watch_likes) = &update_follower_data.watch_likes {
        query = query.bind(watch_likes);
    }

    if let Some(watch_new_hesses) = &update_follower_data.watch_new_hesses {
        query = query.bind(watch_new_hesses);
    }

    if let Some(watch_replies) = &update_follower_data.watch_replies {
        query = query.bind(watch_replies);
    }

    query = query.bind(id);

    match query.execute(&pool).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Err(ApiErrorType::ResourceNotFound(ApiResource::Followers))
            } else {
                Ok(())
            }
        }
        _ => Err(ApiErrorType::InternalServerError),
    }
}

/// Softly deletes a follower relationship from the database.
///
/// This function performs a "soft" delete by setting the `deleted_at` timestamp for the follower relationship
/// with the provided UUID, indicating that the relationship has been logically deleted. The follower relationship's
/// data remains in the database, but it is considered inactive. This approach helps maintain data integrity and
/// allows for potential data recovery if needed.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `id` - The UUID of the follower relationship to delete.
///
/// # Returns
///
/// Returns a `Result<(), ApiErrorType>` indicating success or an `ApiErrorType::InternalServerError` error
/// if any error occurs during database deletion.
/// If no follower relationship with the provided ID is found for deletion or if the relationship is already soft deleted,
/// returns an `ApiErrorType::ResourceNotFound` error.
///
pub async fn delete_follower(pool: Pool<Postgres>, id: Uuid) -> Result<(), ApiErrorType> {
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
                Err(ApiErrorType::ResourceNotFound(ApiResource::Followers))
            } else {
                Ok(())
            }
        }
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}
