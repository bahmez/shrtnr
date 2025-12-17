//! Middleware d'authentification pour Axum.
//!
//! Ce module fournit l'extracteur `AuthUser` qui peut être utilisé
//! dans les handlers pour protéger les routes et obtenir l'utilisateur
//! authentifié automatiquement.

use axum::{
    extract::{Extension, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::Serialize;
use uuid::Uuid;

use crate::backend::{auth::verify_access_token, config::AppState};

/// Structure représentant un utilisateur authentifié.
///
/// Cette structure peut être utilisée comme extracteur dans les handlers Axum
/// pour protéger les routes et obtenir automatiquement l'ID de l'utilisateur
/// authentifié ainsi que l'état de l'application.
///
/// # Utilisation
///
/// Utilisez `AuthUser` comme paramètre dans vos handlers pour protéger une route :
///
/// ```rust,no_run
/// use axum::Json;
/// use shrtnr::backend::middleware::AuthUser;
///
/// pub async fn protected_handler(auth_user: AuthUser) -> Json<String> {
///     Json(format!("User ID: {}", auth_user.user_id))
/// }
/// ```
///
/// Si le token est invalide ou manquant, une erreur 401 Unauthorized
/// sera automatiquement renvoyée.
///
/// # Note
///
/// Cette structure contient également `AppState` pour éviter les conflits
/// lors de l'extraction multiple dans les handlers.
#[derive(Debug, Clone)]
pub struct AuthUser {
    /// ID de l'utilisateur authentifié (extrait du token JWT)
    pub user_id: Uuid,
    /// État de l'application (connexion DB, config JWT, etc.)
    pub state: AppState,
}

/// Erreur d'authentification renvoyée lorsque la validation du token échoue.
#[derive(Debug, Serialize)]
pub struct AuthError {
    /// Message d'erreur descriptif
    pub error: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, Json(self)).into_response()
    }
}

/// Implémentation de `FromRequestParts` pour `AuthUser`.
///
/// Cette implémentation permet d'utiliser `AuthUser` comme extracteur
/// dans les handlers Axum. Elle :
/// 1. Extrait le header `Authorization: Bearer <token>`
/// 2. Vérifie et valide le token JWT
/// 3. Extrait l'ID utilisateur du token
/// 4. Retourne `AuthUser` avec l'état de l'application
///
/// Si l'une de ces étapes échoue, une `AuthError` est renvoyée
/// qui sera automatiquement convertie en réponse HTTP 401.
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extraire le header Authorization
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AuthError {
                    error: "Missing or invalid authorization header".to_string(),
                })?;

        // Extraire l'état de l'application pour accéder à la config JWT
        let Extension(app_state) = Extension::<AppState>::from_request_parts(parts, state)
            .await
            .map_err(|_| AuthError {
                error: "Internal server error".to_string(),
            })?;

        // Vérifier le token
        let claims =
            verify_access_token(bearer.token(), &app_state.jwt_config).map_err(|_| AuthError {
                error: "Invalid or expired token".to_string(),
            })?;

        // Parser l'UUID de l'utilisateur
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AuthError {
            error: "Invalid user ID in token".to_string(),
        })?;

        Ok(AuthUser {
            user_id,
            state: app_state,
        })
    }
}
