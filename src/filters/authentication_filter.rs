use std::{str::FromStr, sync::Arc};

use uuid::Uuid;
use warp::Filter;

use crate::{
    models::User,
    services::users_service::get_user_by_id,
    utils::{
        jwt::{get_claims_from_jwt_token, JwtConfig},
        response::ApiErrorType,
    },
};

/// Creates a Warp filter for JWT-based authentication.
///
/// This function returns a Warp filter that can be used to authenticate incoming requests
/// based on a JSON Web Token (JWT) present in the "Authorization" header. It validates the JWT,
/// retrieves user information from the database, and returns a `User` object if the authentication
/// is successful.
///
/// # Arguments
///
/// * `jwt_config` - An `Arc` reference to the JWT configuration settings.
/// * `pool` - A PostgreSQL database connection pool (`PgPool`) used for querying user data.
///
/// # Returns
///
/// A Warp filter that extracts the authenticated `User` from the request if successful,
/// or rejects the request with appropriate error types.
///
pub fn authentication_filter(
    jwt_config: Arc<JwtConfig>,
    pool: sqlx::postgres::PgPool,
) -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::filters::header::optional("Authorization"))
        .and_then(move |authorization_header: Option<String>| {
            let jwt_config = jwt_config.clone();
            let pool = pool.clone();

            async move {
                // Extract the JWT token from the Authorization header
                let token = match authorization_header {
                    Some(header) if header.starts_with("Bearer ") => header[7..].to_string(),
                    _ => return Err(warp::reject::custom(ApiErrorType::NotLoggedIn)),
                };

                // Retrieve claims from the JWT token
                let claims = get_claims_from_jwt_token(&token, &jwt_config)
                    .ok_or_else(|| warp::reject::custom(ApiErrorType::InvalidJwtToken))?;

                // Parse the user ID from the JWT claims
                let id = Uuid::from_str(&claims.sub)
                    .map_err(|_| warp::reject::custom(ApiErrorType::InternalServerError))?;

                // Retrieve the user from the database using the user ID
                let user = get_user_by_id(pool, id)
                    .await
                    .map_err(|_| warp::reject::custom(ApiErrorType::InvalidJwtToken))?;

                Ok(user)
            }
        })
}
