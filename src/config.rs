use jsonwebtoken::{DecodingKey, EncodingKey};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub jwt_config: JwtConfig,
}

#[derive(Clone)]
pub struct JwtConfig {
    pub encoding_key: Arc<EncodingKey>,
    pub decoding_key: Arc<DecodingKey>,
    pub access_token_lifetime: i64,  // en secondes
    pub refresh_token_lifetime: i64, // en secondes
}

impl JwtConfig {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: Arc::new(EncodingKey::from_secret(secret.as_bytes())),
            decoding_key: Arc::new(DecodingKey::from_secret(secret.as_bytes())),
            access_token_lifetime: 15 * 60,        // 15 minutes
            refresh_token_lifetime: 7 * 24 * 60 * 60, // 7 jours
        }
    }
}

impl AppState {
    pub fn new(db: DatabaseConnection, jwt_secret: &str) -> Self {
        Self {
            db,
            jwt_config: JwtConfig::new(jwt_secret),
        }
    }
}

