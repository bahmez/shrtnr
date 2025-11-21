use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::backend::config::JwtConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // user_id
    pub exp: i64,           // expiration time
    pub iat: i64,           // issued at
    pub token_type: String, // "access" or "refresh"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn create_token_pair(
    user_id: Uuid,
    jwt_config: &JwtConfig,
) -> Result<TokenPair, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now().timestamp();
    let user_id_str = user_id.to_string();

    let access_claims = Claims {
        sub: user_id_str.clone(),
        exp: now + jwt_config.access_token_lifetime,
        iat: now,
        token_type: "access".to_string(),
    };

    let refresh_claims = Claims {
        sub: user_id_str,
        exp: now + jwt_config.refresh_token_lifetime,
        iat: now,
        token_type: "refresh".to_string(),
    };

    let access_token = encode(&Header::default(), &access_claims, &jwt_config.encoding_key)?;
    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &jwt_config.encoding_key,
    )?;

    Ok(TokenPair {
        access_token,
        refresh_token,
    })
}

pub fn verify_access_token(
    token: &str,
    jwt_config: &JwtConfig,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &jwt_config.decoding_key, &validation)?;

    if token_data.claims.token_type != "access" {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ));
    }

    Ok(token_data.claims)
}

pub fn verify_refresh_token(
    token: &str,
    jwt_config: &JwtConfig,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &jwt_config.decoding_key, &validation)?;

    if token_data.claims.token_type != "refresh" {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ));
    }

    Ok(token_data.claims)
}
