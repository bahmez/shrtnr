//! Configuration de l'état de l'application et JWT.
//!
//! Ce module définit les structures de configuration partagées
//! entre les handlers Axum, notamment la connexion à la base de données,
//! la configuration JWT, et le cache des liens.

use crate::backend::entities::shortened_link;
use jsonwebtoken::{DecodingKey, EncodingKey};
use moka::future::Cache;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// État de l'application partagé entre tous les handlers Axum.
///
/// Cette structure contient toutes les dépendances nécessaires
/// pour le fonctionnement de l'application :
/// - La connexion à la base de données
/// - La configuration JWT pour l'authentification
/// - Un cache en mémoire pour les liens raccourcis (optimisation des performances)
///
/// # Thread Safety
///
/// Cette structure est conçue pour être partagée entre plusieurs
/// handlers via `Extension<AppState>` dans Axum. Tous les champs
/// sont thread-safe grâce à l'utilisation de `Arc` et de types
/// thread-safe comme `DatabaseConnection`.
///
/// # Exemple
///
/// ```rust,no_run
/// use shrtnr::backend::{connect, AppState};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let db = connect().await?;
/// let jwt_secret = std::env::var("JWT_SECRET")
///     .unwrap_or_else(|_| "default-secret".to_string());
/// let app_state = AppState::new(db, &jwt_secret);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct AppState {
    /// Connexion à la base de données (SQLite ou PostgreSQL)
    pub db: DatabaseConnection,
    /// Configuration pour la génération et validation des tokens JWT
    pub jwt_config: JwtConfig,
    /// Cache en mémoire pour les liens raccourcis
    ///
    /// Le cache stocke jusqu'à 10 000 entrées avec un TTL de 1 heure
    /// pour améliorer les performances lors des redirections fréquentes.
    pub link_cache: Cache<String, shortened_link::Model>,
}

/// Configuration pour la gestion des tokens JWT.
///
/// Cette structure contient les clés de codage/décodage et les durées
/// de vie des tokens d'accès et de rafraîchissement.
#[derive(Clone)]
pub struct JwtConfig {
    /// Clé utilisée pour encoder (signer) les tokens JWT
    pub encoding_key: Arc<EncodingKey>,
    /// Clé utilisée pour décoder (vérifier) les tokens JWT
    pub decoding_key: Arc<DecodingKey>,
    /// Durée de vie du token d'accès en secondes (par défaut: 15 minutes)
    pub access_token_lifetime: i64,
    /// Durée de vie du token de rafraîchissement en secondes (par défaut: 7 jours)
    pub refresh_token_lifetime: i64,
}

impl JwtConfig {
    /// Crée une nouvelle configuration JWT à partir d'un secret.
    ///
    /// # Arguments
    ///
    /// * `secret` - Le secret utilisé pour signer et vérifier les tokens
    ///
    /// # Durées de vie par défaut
    ///
    /// - Access token : 15 minutes
    /// - Refresh token : 7 jours
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use shrtnr::backend::config::JwtConfig;
    ///
    /// let config = JwtConfig::new("my-secret-key");
    /// assert_eq!(config.access_token_lifetime, 15 * 60);
    /// assert_eq!(config.refresh_token_lifetime, 7 * 24 * 60 * 60);
    /// ```
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
    /// Crée une nouvelle instance d'`AppState`.
    ///
    /// Initialise l'état de l'application avec :
    /// - La connexion à la base de données fournie
    /// - Une configuration JWT créée à partir du secret
    /// - Un cache de liens avec une capacité de 10 000 entrées et un TTL de 1 heure
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données (SQLite ou PostgreSQL)
    /// * `jwt_secret` - Secret utilisé pour signer et vérifier les tokens JWT
    ///
    /// # Exemple
    ///
    /// ```rust,no_run
    /// use shrtnr::backend::{connect, AppState};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let db = connect().await?;
    /// let jwt_secret = std::env::var("JWT_SECRET")
    ///     .unwrap_or_else(|_| "development-secret".to_string());
    /// let app_state = AppState::new(db, &jwt_secret);
    /// # Ok(())
    /// # }
    /// ```
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
