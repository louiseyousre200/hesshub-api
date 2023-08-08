use std::convert::Infallible;

use warp::{hyper::StatusCode, reject::MethodNotAllowed};

use super::response::{ApiErrorResponse, ApiErrorType};

fn create_err_reply<T: serde::Serialize>(
    status_code: StatusCode,
    code: &str,
    details: Option<T>,
) -> warp::reply::WithStatus<warp::reply::Json> {
    let error = ApiErrorResponse {
        code: code.to_string(),
        details,
    };

    let err_reply_json = warp::reply::json(&error);

    let err_reply = warp::reply::with_status(err_reply_json, status_code);

    err_reply
}

fn create_not_found_err_reply() -> warp::reply::WithStatus<warp::reply::Json> {
    create_err_reply::<()>(StatusCode::NOT_FOUND, "ROUTE_NOT_FOUND", None)
}

pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    if err.is_not_found() {
        Ok(create_not_found_err_reply())
    } else {
        if let Some(e) = err.find::<ApiErrorType>() {
            Ok(create_err_reply(
                e.status_code(),
                e.code(),
                Some(&e.details()),
            ))
        } else if let Some(_) = err.find::<MethodNotAllowed>() {
            Ok(create_not_found_err_reply())
        } else {
            Ok(create_err_reply::<()>(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                None,
            ))
        }
    }
}
