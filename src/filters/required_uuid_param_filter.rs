use uuid::Uuid;
use warp::Filter;

use crate::utils::response::{ApiErrorType, ApiResource};

async fn validate_uuid(uuid: String, resource: ApiResource) -> Result<Uuid, ApiErrorType> {
    match uuid::Uuid::parse_str(uuid.as_str()) {
        Ok(uuid) => Ok(uuid),
        Err(_) => Err(ApiErrorType::InvalidIdParam(resource)),
    }
}

pub fn required_uuid_param_filter(
    resource: ApiResource,
) -> impl Filter<Extract = (Uuid,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::path::param::<String>())
        .and_then(move |id: String| {
            let resource = resource.clone();

            async move {
                let id = validate_uuid(id, resource)
                    .await
                    .map_err(warp::reject::custom)?;

                Ok::<Uuid, warp::Rejection>(id)
            }
        })
}
