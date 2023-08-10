use std::{env, sync::Arc};

use handlebars::Handlebars;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::{pool::PoolOptions, Pool, Postgres};
use utils::handle_rejection;
use warp::Filter;

use crate::{
    emails_data::CommonEmailDetails,
    utils::{jwt::JwtConfig, TokensConfig},
};

mod emails_data;
mod filters;
mod handlers;
mod models;
mod models_validators;
mod services;
mod utils;

async fn init_app() -> (
    Arc<JwtConfig>,
    Pool<Postgres>,
    Arc<Handlebars<'static>>,
    Arc<TokensConfig>,
    Arc<CommonEmailDetails>,
) {
    log::info!("🔑 Getting the environment variables as documented 🔑");

    let database_url = env::var("DATABASE_URL").unwrap();

    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let jwt_expire_in_hours = env::var("JWT_EXPIRE_IN_HOURS")
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let account_activation_token_expire_in_hours =
        env::var("ACCOUNT_ACTIVATION_TOKEN_EXPIRE_IN_HOURS")
            .unwrap()
            .parse::<i64>()
            .unwrap();

    let password_reset_token_expire_in_hours = env::var("PASSWORD_RESET_TOKEN_EXPIRE_IN_HOURS")
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let tokens_config = Arc::new(TokensConfig {
        account_activation_token_expire_in_hours,
        password_reset_token_expire_in_hours,
    });

    let facebook_link = env::var("FACEBOOK_LINK").ok();
    let twitter_link = env::var("TWITTER_LINK").ok();
    let instagram_link = env::var("INSTAGRAM_LINK").ok();
    let linked_in_link = env::var("LINKED_IN_LINK").ok();

    let first_contatct_line = env::var("FIRST_CONTACT_LINE").ok();
    let second_contatct_line = env::var("SECOND_CONTACT_LINE").ok();

    let common_email_details = Arc::new(CommonEmailDetails {
        facebook_link,
        first_contatct_line,
        instagram_link,
        linked_in_link,
        second_contatct_line,
        twitter_link,
    });

    log::info!("🔧 Creating a handlebars registry for the templates 🔧");

    let mut hb: Handlebars<'_> = Handlebars::new();

    hb.register_template_file(
        "user-activation-email",
        "./email_templates/user_activation_email.hbs",
    )
    .unwrap();

    let hb: Arc<Handlebars<'_>> = Arc::new(hb);

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

    (jwt_config, pool, hb, tokens_config, common_email_details)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("✨ Initializing dotenv ✨");

    dotenv::dotenv().ok();

    let (jwt_config, pool, hb, tokens_config, common_email_details) = init_app().await;

    let include_jwt_config = warp::any().map(move || jwt_config.clone());

    let include_pool = warp::any().map(move || pool.clone());

    let include_handlebars = warp::any().map(move || hb.clone());

    let include_tokens_config = warp::any().map(move || tokens_config.clone());

    let include_common_email_details = warp::any().map(move || common_email_details.clone());

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
