use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use super::response::ApiErrorType;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct JwtConfig {
    pub private_key: EncodingKey,
    pub public_key: DecodingKey,
    pub expire_in_hours: i64,
}

pub fn generate_jwt_token(user_id: Uuid, jwt_config: &JwtConfig) -> Result<String, ApiErrorType> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::hours(jwt_config.expire_in_hours)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    encode(&Header::default(), &claims, &jwt_config.private_key)
        .map_err(|_| ApiErrorType::InternalServerError)
}

pub fn get_claims_from_jwt_token(token: &String, jwt_config: &JwtConfig) -> Option<TokenClaims> {
    match decode::<TokenClaims>(token, &jwt_config.public_key, &Validation::default()) {
        Ok(c) => Some(c.claims),
        Err(_) => None,
    }
}
