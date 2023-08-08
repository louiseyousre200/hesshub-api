use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    models::{Gender, User, UserRole},
    models_validators::user_validator::{InsertUserData, UpdateUserData},
    utils::response::{ApiErrorType, ApiResource},
};

/// Retrieves a user by their ID from the database.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `id` - The UUID of the user to retrieve.
///
/// # Returns
///
/// Returns a `Result` containing the retrieved `User` if successful.
/// If no user is found, returns an `ApiErrorType::ResourceNotFound` error.
/// If any other error occurs during the database query, returns an `ApiErrorType::InternalServerError` error.
///s
pub async fn get_user_by_id(pool: Pool<Postgres>, id: Uuid) -> Result<User, ApiErrorType> {
    let query_result = sqlx::query_as!(
        User,
        r#"SELECT
            id,
            name,
            gender AS "gender: Gender",
            role AS "role: UserRole",
            bio,
            user_profile_image_id,
            email,
            username,
            password,
            activated,
            created_at,
            updated_at,
            deleted_at,
            verified,
            verified_at,
            verified_by
        FROM users WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await;

    match query_result {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(ApiErrorType::ResourceNotFound(ApiResource::Users)),
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}

/// Inserts a new user into the database.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `insert_user_data` - Data to insert for the new user.
/// * `user_profile_image_id` - Optional UUID of the user's profile image.
/// * `verified_by` - Optional UUID of the user who verified this user.
///
/// # Returns
///
/// Returns a `Result` containing the inserted `User` if successful.
/// If the user (email, username) already exists, returns an `ApiErrorType::AlreadyExists` error.
/// If any other error occurs during database insertion, returns an `ApiErrorType::InternalServerError` error.
///
pub async fn insert_user(
    pool: Pool<Postgres>,
    insert_user_data: InsertUserData,
    user_profile_image_id: Option<Uuid>,
    verified_by: Option<Uuid>,
) -> Result<User, ApiErrorType> {
    let verified_at = if let Some(_) = verified_by {
        Some(Utc::now())
    } else {
        None
    };

    let query_result = sqlx::query_as!(
        User,
        r#"INSERT INTO users (
            name,
            gender,
            role,
            bio,
            email,
            user_profile_image_id,
            username,
            password,
            activated,
            verified,
            verified_at,
            verified_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING
            id,
            name,
            gender AS "gender: Gender",
            role AS "role: UserRole",
            bio,
            user_profile_image_id,
            email,
            username,
            password,
            activated,
            created_at,
            updated_at,
            deleted_at,
            verified,
            verified_at,
            verified_by
        "#,
        insert_user_data.name,
        insert_user_data.gender as Gender,
        insert_user_data.role as UserRole,
        insert_user_data.bio,
        insert_user_data.email,
        user_profile_image_id,
        insert_user_data.username,
        insert_user_data.password,
        insert_user_data.activated,
        insert_user_data.verified,
        verified_at,
        verified_by
    )
    .fetch_one(&pool)
    .await;

    match query_result {
        Ok(user) => Ok(user),
        Err(e) => match e {
            sqlx::Error::Database(e) if e.is_unique_violation() => {
                Err(ApiErrorType::AlreadyExists(ApiResource::Users))
            }
            _ => Err(ApiErrorType::InternalServerError),
        },
    }
}

/// (Softly) Deletes a user from the database.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `id` - The UUID of the user to delete.
///
/// # Returns
///
/// Returns `Result<(), ApiErrorType>` indicating success or an `ApiErrorType::InternalServerError` error
/// if any error occurs during database deletion.
/// If no user with the provided ID and isn't deleted is found for deletion, returns an `ApiErrorType::ResourceNotFound` error.
///
pub async fn delete_user(pool: Pool<Postgres>, id: Uuid) -> Result<(), ApiErrorType> {
    let current_date = Utc::now();

    let query_result = sqlx::query!(
        "UPDATE users SET deleted_at = $1 WHERE id = $2 AND deleted_at IS NULL",
        current_date,
        id,
    )
    .execute(&pool)
    .await;

    match query_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Err(ApiErrorType::ResourceNotFound(ApiResource::Users))
            } else {
                Ok(())
            }
        }
        Err(_) => Err(ApiErrorType::InternalServerError),
    }
}

/// Updates a user's information in the database.
///
/// # Arguments
///
/// * `pool` - A database connection pool.
/// * `user_id` - The ID of the user to update.
/// * `update_user_data` - Data containing the fields to update.
///
/// # Returns
///
/// Returns `Result<(), ApiErrorType>` indicating success or an `ApiErrorType::InternalServerError` error
/// if any error occurs during database update.
/// If no user with the provided ID and isn't deleted is found for updating, returns an `ApiErrorType::ResourceNotFound` error.
///
pub async fn update_user(
    pool: Pool<Postgres>,
    user_id: i32,
    update_user_data: UpdateUserData,
) -> Result<(), ApiErrorType> {
    let mut update_set = vec![];
    let mut counter = 1;

    if let Some(name) = &update_user_data.name {
        update_set.push(format!("name = ${}", name));
        counter += 1;
    }

    if let Some(gender) = &update_user_data.gender {
        update_set.push(format!(
            "gender = ${}",
            serde_json::to_string(gender).unwrap()
        ));
        counter += 1;
    }

    if let Some(role) = &update_user_data.role {
        update_set.push(format!("role = ${}", serde_json::to_string(role).unwrap()));
        counter += 1;
    }

    if let Some(bio) = &update_user_data.bio {
        update_set.push(format!("bio = ${}", bio));
        counter += 1;
    }

    if let Some(email) = &update_user_data.email {
        update_set.push(format!("email = ${}", email));
        counter += 1;
    }

    if let Some(username) = &update_user_data.username {
        update_set.push(format!("username = ${}", username));
        counter += 1;
    }

    if let Some(password) = &update_user_data.password {
        update_set.push(format!("password = ${}", password));
        counter += 1;
    }

    if let Some(activated) = &update_user_data.activated {
        update_set.push(format!("activated = ${}", activated));
        counter += 1;
    }

    if let Some(verified) = &update_user_data.verified {
        update_set.push(format!("verified = ${}", verified));
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

    if let Some(name) = &update_user_data.name {
        query = query.bind(name);
    }

    if let Some(gender) = &update_user_data.gender {
        query = query.bind(gender);
    }

    if let Some(role) = &update_user_data.role {
        query = query.bind(role);
    }

    if let Some(bio) = &update_user_data.bio {
        query = query.bind(bio);
    }

    if let Some(email) = &update_user_data.email {
        query = query.bind(email);
    }

    if let Some(username) = &update_user_data.username {
        query = query.bind(username);
    }

    if let Some(password) = &update_user_data.password {
        query = query.bind(password);
    }

    if let Some(activated) = &update_user_data.activated {
        query = query.bind(activated);
    }

    if let Some(verified) = &update_user_data.verified {
        query = query.bind(verified);
    }

    query = query.bind(user_id);

    match query.execute(&pool).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Err(ApiErrorType::ResourceNotFound(ApiResource::Users))
            } else {
                Ok(())
            }
        }
        _ => Err(ApiErrorType::InternalServerError),
    }
}
