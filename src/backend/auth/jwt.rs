//! Gestion des tokens JWT pour l'authentification.
//!
//! Ce module fournit les fonctions pour créer, valider et gérer les tokens JWT
//! utilisés pour l'authentification des utilisateurs.

use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::backend::config::JwtConfig;

/// Claims (revendications) contenues dans un token JWT.
///
/// Les claims contiennent les informations sur l'utilisateur et le token lui-même.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject : ID de l'utilisateur (UUID sous forme de string)
    pub sub: String,
    /// Expiration : Timestamp Unix de l'expiration du token
    pub exp: i64,
    /// Issued At : Timestamp Unix de la création du token
    pub iat: i64,
    /// Type de token : "access" ou "refresh"
    pub token_type: String,
}

/// Paire de tokens JWT (access et refresh).
///
/// Utilisée lors de l'authentification pour retourner les deux tokens
/// nécessaires au fonctionnement de l'API.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    /// Token d'accès (durée de vie courte, par défaut 15 minutes)
    pub access_token: String,
    /// Token de rafraîchissement (durée de vie longue, par défaut 7 jours)
    pub refresh_token: String,
}

/// Crée une paire de tokens JWT (access et refresh) pour un utilisateur.
///
/// Génère deux tokens avec des durées de vie différentes :
/// - Access token : courte durée (15 minutes par défaut)
/// - Refresh token : longue durée (7 jours par défaut)
///
/// # Arguments
///
/// * `user_id` - L'identifiant unique de l'utilisateur
/// * `jwt_config` - La configuration JWT contenant les clés et durées de vie
///
/// # Returns
///
/// * `Ok(TokenPair)` - La paire de tokens générée
/// * `Err(Error)` - En cas d'erreur lors de l'encodage
///
/// # Exemple
///
/// ```rust,no_run
/// use uuid::Uuid;
/// use shrtnr::backend::{auth::create_token_pair, config::JwtConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let user_id = Uuid::new_v4();
/// let jwt_config = JwtConfig::new("secret-key");
/// let tokens = create_token_pair(user_id, &jwt_config)?;
/// println!("Access token: {}", tokens.access_token);
/// # Ok(())
/// # }
/// ```
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

/// Vérifie et valide un token d'accès JWT.
///
/// Décode le token, vérifie sa signature et s'assure qu'il s'agit bien
/// d'un token d'accès (et non un refresh token).
///
/// # Arguments
///
/// * `token` - Le token JWT à vérifier
/// * `jwt_config` - La configuration JWT contenant la clé de décodage
///
/// # Returns
///
/// * `Ok(Claims)` - Les claims du token si valide
/// * `Err(Error)` - Si le token est invalide, expiré, ou n'est pas un access token
///
/// # Exemple
///
/// ```rust,no_run
/// use shrtnr::backend::{auth::verify_access_token, config::JwtConfig};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let jwt_config = JwtConfig::new("secret-key");
/// let claims = verify_access_token("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...", &jwt_config)?;
/// println!("User ID: {}", claims.sub);
/// # Ok(())
/// # }
/// ```
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

/// Vérifie et valide un token de rafraîchissement JWT.
///
/// Décode le token, vérifie sa signature et s'assure qu'il s'agit bien
/// d'un refresh token (et non un access token).
///
/// # Arguments
///
/// * `token` - Le refresh token JWT à vérifier
/// * `jwt_config` - La configuration JWT contenant la clé de décodage
///
/// # Returns
///
/// * `Ok(Claims)` - Les claims du token si valide
/// * `Err(Error)` - Si le token est invalide, expiré, ou n'est pas un refresh token
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
