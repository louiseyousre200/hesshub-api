use utils::handle_rejection;
use warp::Filter;

mod filters;
mod handlers;
mod models;
mod models_validators;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    let api_v1_routes = warp::path("api")
        .and(warp::path("v1"))
        .map(|| "Hello, World!");

    warp::serve(api_v1_routes.recover(handle_rejection))
        .run(([127, 0, 0, 1], 6060))
        .await;
}
