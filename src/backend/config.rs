use crate::backend::entities::shortened_link;
use jsonwebtoken::{DecodingKey, EncodingKey};
use moka::future::Cache;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub jwt_config: JwtConfig,
    pub link_cache: Cache<String, shortened_link::Model>,
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
            access_token_lifetime: 15 * 60,           // 15 minutes
            refresh_token_lifetime: 7 * 24 * 60 * 60, // 7 jours
        }
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("db", &"<DatabaseConnection>")
            .field("jwt_config", &"<JwtConfig>")
            .field("link_cache", &"<Cache>")
            .finish()
    }
}

impl AppState {
    pub fn new(db: DatabaseConnection, jwt_secret: &str) -> Self {
        // Create cache with 10,000 max entries and 1 hour TTL
        let link_cache = Cache::builder()
            .max_capacity(10_000)
            .time_to_live(std::time::Duration::from_secs(3600))
            .build();

        Self {
            db,
            jwt_config: JwtConfig::new(jwt_secret),
            link_cache,
        }
    }
}
