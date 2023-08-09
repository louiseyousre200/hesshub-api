use uuid::Uuid;
use warp::Filter;

use crate::utils::response::{ApiErrorType, ApiResource};

/// Validate a UUID string and convert it to a `Uuid`.
///
/// This function takes a UUID string and attempts to parse it into a `Uuid` instance.
/// If the parsing is successful, the resulting `Uuid` is returned. If parsing fails,
/// an `ApiErrorType::InvalidIdParam` error is returned, indicating that the provided
/// UUID string is invalid for the specified resource.
///
/// # Parameters
///
/// - `uuid`: A string representing a UUID to be parsed.
/// - `resource`: An `ApiResource` enum variant indicating the context of the UUID validation.
///
/// # Returns
///
/// A `Result` containing either the parsed `Uuid` or an `ApiErrorType` indicating the error.
///
fn validate_uuid(uuid: String, resource: ApiResource) -> Result<Uuid, ApiErrorType> {
    match uuid::Uuid::parse_str(uuid.as_str()) {
        Ok(uuid) => Ok(uuid),
        Err(_) => Err(ApiErrorType::InvalidIdParam(resource)),
    }
}

/// Create a Warp filter for extracting and validating a required UUID path parameter.
///
/// This function returns a filter that can be used in Warp routes to extract and validate
/// a required UUID path parameter. It uses the `validate_uuid` function to validate the UUID
/// string and convert it into a `Uuid` instance. If validation is successful, the `Uuid` is
/// returned; otherwise, an appropriate rejection is generated.
///
/// # Parameters
///
/// - `resource`: An `ApiResource` enum variant indicating the context of the UUID validation.
///
/// # Returns
///
/// A Warp filter that extracts, validates, and converts the UUID path parameter into a `Uuid`.
///
pub fn required_uuid_param_filter(
    resource: ApiResource,
) -> impl Filter<Extract = (Uuid,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::path::param::<String>())
        .and_then(move |id: String| {
            let resource = resource.clone();

            async move {
                let id = validate_uuid(id, resource).map_err(warp::reject::custom)?;

                Ok::<Uuid, warp::Rejection>(id)
            }
        })
}
