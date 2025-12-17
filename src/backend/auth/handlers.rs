//! Handlers API pour l'authentification et la gestion des utilisateurs.
//!
//! Ce module contient tous les handlers Axum pour les endpoints d'authentification :
//! - Inscription de nouveaux utilisateurs
//! - Connexion et génération de tokens JWT
//! - Rafraîchissement des tokens
//! - Récupération et mise à jour du profil utilisateur
//! - Déconnexion

use axum::{extract::Extension, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{create_token_pair, hash_password, verify_password, verify_refresh_token};
use crate::backend::{config::AppState, middleware::AuthUser, repositories::UserRepository};
use crate::shared::responses::{ApiError, ApiMessage};

// ===== Request/Response Types =====

/// Requête pour l'inscription d'un nouvel utilisateur.
///
/// Tous les champs sont requis sauf `name` qui est optionnel.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Adresse email de l'utilisateur (doit être unique dans le système)
    pub email: String,
    /// Mot de passe en clair (sera hashé avec bcrypt avant stockage)
    pub password: String,
    /// Nom complet de l'utilisateur (optionnel)
    pub name: Option<String>,
}

/// Requête pour la connexion d'un utilisateur existant.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Adresse email de l'utilisateur
    pub email: String,
    /// Mot de passe en clair (sera comparé avec le hash stocké)
    pub password: String,
}

/// Requête pour rafraîchir un token d'accès expiré.
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    /// Token de rafraîchissement valide
    pub refresh_token: String,
}

/// Requête pour mettre à jour le profil utilisateur.
#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    /// Nouveau nom de l'utilisateur (optionnel, laisse inchangé si `None`)
    pub name: Option<String>,
    /// Nouvelle adresse email (optionnel, doit être unique si fourni)
    pub email: Option<String>,
}

/// Réponse d'authentification contenant les informations utilisateur et les tokens JWT.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// Informations de l'utilisateur authentifié
    pub user: UserResponse,
    /// Token JWT d'accès (durée de vie : 15 minutes)
    pub access_token: String,
    /// Token JWT de rafraîchissement (durée de vie : 7 jours)
    pub refresh_token: String,
}

/// Réponse contenant les informations d'un utilisateur.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// Identifiant unique de l'utilisateur (UUID)
    pub id: String,
    /// Adresse email de l'utilisateur
    pub email: String,
    /// Nom complet de l'utilisateur (peut être `None`)
    pub name: Option<String>,
    /// Date de création du compte (format ISO 8601, peut être `None`)
    pub created_at: Option<String>,
}

// ===== Handlers =====

/// Handler pour l'inscription d'un nouvel utilisateur.
///
/// Crée un compte utilisateur avec un email et un mot de passe hashé,
/// puis génère une paire de tokens JWT (access et refresh) pour l'authentification.
///
/// # Endpoint
///
/// `POST /api/auth/register`
///
/// # Arguments
///
/// * `state` - L'état de l'application contenant la connexion DB et la config JWT
/// * `payload` - Les données d'inscription (email, password, name optionnel)
///
/// # Returns
///
/// Retourne une `AuthResponse` contenant :
/// - Les informations de l'utilisateur créé
/// - Un access token JWT (valide 15 minutes)
/// - Un refresh token JWT (valide 7 jours)
///
/// # Errors
///
/// * `409 Conflict` - Si l'email est déjà utilisé
/// * `500 Internal Server Error` - En cas d'erreur de hashage, création en DB, ou génération de tokens
///
/// # Exemple de requête
///
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "securepassword123",
///   "name": "John Doe"
/// }
/// ```
pub async fn register_handler(
    Extension(state): Extension<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    // Vérifier si l'email existe déjà
    if let Ok(Some(_)) = UserRepository::find_by_email(&state.db, &payload.email).await {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiError::new("Email already exists")),
        ));
    }

    // Hasher le mot de passe
    let password_hash = hash_password(&payload.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new("Failed to hash password")),
        )
    })?;

    // Créer l'utilisateur
    let user = UserRepository::create(&state.db, payload.email, password_hash, payload.name)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new(format!("Failed to create user: {}", e))),
            )
        })?;

    // Créer les tokens JWT
    let tokens = create_token_pair(user.id, &state.jwt_config).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new("Failed to create tokens")),
        )
    })?;

    Ok(Json(AuthResponse {
        user: UserResponse {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            created_at: user.created_at.map(|dt| dt.to_string()),
        },
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    }))
}

/// Handler pour la connexion d'un utilisateur existant.
///
/// Authentifie un utilisateur avec son email et mot de passe, puis génère
/// une paire de tokens JWT si les credentials sont valides.
///
/// # Endpoint
///
/// `POST /api/auth/login`
///
/// # Arguments
///
/// * `state` - L'état de l'application
/// * `payload` - Les credentials de connexion (email et password)
///
/// # Returns
///
/// Retourne une `AuthResponse` avec les tokens JWT et les infos utilisateur.
///
/// # Errors
///
/// * `401 Unauthorized` - Si l'email n'existe pas ou si le mot de passe est incorrect
/// * `500 Internal Server Error` - En cas d'erreur de base de données ou de génération de tokens
///
/// # Sécurité
///
/// Pour des raisons de sécurité, le message d'erreur est générique ("Invalid credentials")
/// même si l'email n'existe pas, pour éviter l'énumération d'emails.
pub async fn login_handler(
    Extension(state): Extension<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    // Trouver l'utilisateur par email
    let user = UserRepository::find_by_email(&state.db, &payload.email)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new(format!("Database error: {}", e))),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiError::new("Invalid credentials")),
            )
        })?;

    // Vérifier le mot de passe
    let is_valid = verify_password(&payload.password, &user.password_hash).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new("Failed to verify password")),
        )
    })?;

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiError::new("Invalid credentials")),
        ));
    }

    // Créer les tokens JWT
    let tokens = create_token_pair(user.id, &state.jwt_config).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new("Failed to create tokens")),
        )
    })?;

    Ok(Json(AuthResponse {
        user: UserResponse {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            created_at: user.created_at.map(|dt| dt.to_string()),
        },
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    }))
}

/// Handler pour la déconnexion d'un utilisateur.
///
/// Dans une implémentation JWT stateless, le logout est généralement géré côté client
/// en supprimant les tokens du localStorage. Ce handler retourne simplement un message
/// de confirmation.
///
/// # Endpoint
///
/// `POST /api/auth/logout`
///
/// # Returns
///
/// Un message de confirmation de déconnexion.
///
/// # Note
///
/// Pour une invalidation côté serveur, il faudrait implémenter une blacklist de tokens
/// ou un système de révocation. Actuellement, les tokens restent valides jusqu'à leur expiration.
pub async fn logout_handler() -> Json<ApiMessage> {
    // Dans une implémentation JWT stateless, le logout est généralement géré côté client
    // en supprimant les tokens. Pour une invalidation côté serveur, vous auriez besoin
    // d'une blacklist de tokens ou d'un système de révocation.
    Json(ApiMessage::new("Logged out successfully"))
}

/// Handler pour rafraîchir un token d'accès expiré.
///
/// Utilise un refresh token valide pour générer une nouvelle paire de tokens
/// (access et refresh) sans nécessiter de nouvelles credentials.
///
/// # Endpoint
///
/// `POST /api/auth/refresh-token`
///
/// # Arguments
///
/// * `state` - L'état de l'application
/// * `payload` - Le refresh token à utiliser
///
/// # Returns
///
/// Retourne une nouvelle `AuthResponse` avec de nouveaux tokens JWT.
///
/// # Errors
///
/// * `401 Unauthorized` - Si le refresh token est invalide ou expiré
/// * `500 Internal Server Error` - En cas d'erreur de base de données ou de génération de tokens
pub async fn refresh_token_handler(
    Extension(state): Extension<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    // Vérifier le refresh token
    let claims = verify_refresh_token(&payload.refresh_token, &state.jwt_config).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiError::new("Invalid refresh token")),
        )
    })?;

    // Récupérer l'utilisateur
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new("Invalid user ID in token")),
        )
    })?;

    let user = UserRepository::find_by_id(&state.db, user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new(format!("Database error: {}", e))),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiError::new("User not found")),
            )
        })?;

    // Créer de nouveaux tokens
    let tokens = create_token_pair(user.id, &state.jwt_config).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new("Failed to create tokens")),
        )
    })?;

    Ok(Json(AuthResponse {
        user: UserResponse {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            created_at: user.created_at.map(|dt| dt.to_string()),
        },
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    }))
}

/// Handler pour récupérer les informations de l'utilisateur authentifié.
///
/// Retourne les informations du profil de l'utilisateur actuellement authentifié,
/// identifié via le token JWT dans le header Authorization.
///
/// # Endpoint
///
/// `GET /api/auth/me`
///
/// # Arguments
///
/// * `auth_user` - L'utilisateur authentifié (extrait automatiquement du token JWT)
///
/// # Returns
///
/// Les informations de l'utilisateur authentifié.
///
/// # Errors
///
/// * `401 Unauthorized` - Si le token est invalide ou manquant (géré par `AuthUser`)
/// * `404 Not Found` - Si l'utilisateur n'existe plus en base de données
/// * `500 Internal Server Error` - En cas d'erreur de base de données
pub async fn me_handler(
    auth_user: AuthUser,
) -> Result<Json<UserResponse>, (StatusCode, Json<ApiError>)> {
    let state = &auth_user.state;
    let user = UserRepository::find_by_id(&state.db, auth_user.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new(format!("Database error: {}", e))),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(ApiError::new("User not found"))))?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        created_at: user.created_at.map(|dt| dt.to_string()),
    }))
}

/// Handler pour mettre à jour le profil de l'utilisateur authentifié.
///
/// Permet de modifier le nom et/ou l'email de l'utilisateur. L'email doit être unique
/// s'il est fourni.
///
/// # Endpoint
///
/// `PUT /api/auth/profile`
///
/// # Arguments
///
/// * `auth_user` - L'utilisateur authentifié
/// * `payload` - Les champs à mettre à jour (name et/ou email, optionnels)
///
/// # Returns
///
/// Les informations mises à jour de l'utilisateur.
///
/// # Errors
///
/// * `401 Unauthorized` - Si le token est invalide ou manquant (géré par `AuthUser`)
/// * `409 Conflict` - Si le nouvel email est déjà utilisé par un autre utilisateur
/// * `500 Internal Server Error` - En cas d'erreur de base de données
pub async fn update_profile_handler(
    auth_user: AuthUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ApiError>)> {
    let state = &auth_user.state;
    // Si l'email est fourni, vérifier qu'il n'est pas déjà utilisé
    if let Some(ref email) = payload.email {
        if let Ok(Some(existing_user)) = UserRepository::find_by_email(&state.db, email).await {
            if existing_user.id != auth_user.user_id {
                return Err((
                    StatusCode::CONFLICT,
                    Json(ApiError::new("Email already in use")),
                ));
            }
        }
    }

    let user = UserRepository::update(&state.db, auth_user.user_id, payload.name, payload.email)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new(format!("Failed to update user: {}", e))),
            )
        })?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        created_at: user.created_at.map(|dt| dt.to_string()),
    }))
}
