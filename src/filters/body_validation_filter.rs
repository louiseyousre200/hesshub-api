use std::collections::HashMap;

use warp::{Buf, Filter};

use crate::utils::response::{ApiErrorType, ValidationError};

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
