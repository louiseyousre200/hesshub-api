use std::collections::HashMap;

use warp::{Buf, Filter};

use crate::utils::response::{ApiErrorType, ValidationError};

/// Create a Warp filter for JSON body validation.
///
/// This function returns a filter that can be used in Warp routes to validate and
/// process JSON request bodies. It extracts the request body, deserializes it into
/// a `HashMap<String, serde_json::Value>`, and then attempts to convert the `HashMap`
/// into a specified type `T` using the `TryFrom` trait.
///
/// # Type Parameters
///
/// - `T`: The type to convert the deserialized JSON data into. This type must implement
///   the `TryFrom<HashMap<String, serde_json::Value>>` trait.
///
/// # Returns
///
/// A Warp filter that extracts, validates, and converts the JSON request body into
/// the specified type `T`.
///
pub fn body_validation_filter<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: TryFrom<HashMap<String, serde_json::Value>>,
    <T as TryFrom<HashMap<std::string::String, serde_json::Value>>>::Error: warp::reject::Reject,
{
    warp::any()
        .and(warp::body::bytes())
        .and_then(|mut buf: warp::hyper::body::Bytes| async move {
            let res = serde_json::from_slice::<HashMap<String, serde_json::Value>>(
                &buf.copy_to_bytes(buf.remaining()),
            );

            match res {
                Ok(data) => match T::try_from(data) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(warp::reject::custom(err)),
                },
                Err(_) => Err(warp::reject::custom(ApiErrorType::BodyValidationErrors(
                    vec![ValidationError::InvalidJsonBody],
                ))),
            }
        })
}
