use std::{env, sync::Arc};

use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::{pool::PoolOptions, Pool, Postgres};
use utils::handle_rejection;
use warp::Filter;

use crate::utils::jwt::JwtConfig;

mod filters;
mod handlers;
mod models;
mod models_validators;
mod services;
mod utils;

async fn init_app() -> (Arc<JwtConfig>, Pool<Postgres>) {
    log::info!(
        "🔑 Getting the environment variables JWT_SECRET and JWT_EXPIRE_IN_HOURS and DATABASE_URL 🔑"
    );

    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let database_url = env::var("DATABASE_URL").unwrap();
    let jwt_expire_in_hours = env::var("JWT_EXPIRE_IN_HOURS")
        .unwrap()
        .parse::<i64>()
        .unwrap();

    log::info!("🔧 Creating a connection pool to the database 🔧");

    let pool_options = PoolOptions::new().max_connections(100);

    log::info!("🔧 Running necessary database migrations 🔧");

    let pool = pool_options.connect(&database_url).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    log::info!("🔑 Preparing JWT configuration 🔑");

    let private_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let public_key = DecodingKey::from_secret(jwt_secret.as_bytes());

    let jwt_config = Arc::new(JwtConfig {
        private_key: private_key.clone(),
        public_key,
        expire_in_hours: jwt_expire_in_hours,
    });

    (jwt_config, pool)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("✨ Initializing dotenv ✨");

    dotenv::dotenv().ok();

    let (jwt_config, pool) = init_app().await;

    let include_jwt_config = warp::any().map(move || jwt_config.clone());

    let include_pool = warp::any().map(move || pool.clone());

    log::info!("🚀 Finished preparing the app 🚀");

    log::info!("🔧 Initializing the filters (routes) 🔧");

    let api_v1_routes = warp::path("api")
        .and(warp::path("v1"))
        .map(|| "Hello, World!");

    log::info!("🚀 Starting the server 🚀");

    warp::serve(api_v1_routes.recover(handle_rejection))
        .run(([127, 0, 0, 1], 6060))
        .await;
}
