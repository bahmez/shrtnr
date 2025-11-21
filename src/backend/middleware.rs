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

/// Structure représentant un utilisateur authentifié
/// Cette structure peut être utilisée comme extracteur dans les handlers
/// It also carries the AppState to avoid double extraction conflicts
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub state: AppState,
}

#[derive(Debug, Serialize)]
pub struct AuthError {
    pub error: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, Json(self)).into_response()
    }
}

/// Implémentation de FromRequestParts pour AuthUser
/// Cela permet d'utiliser AuthUser comme extracteur dans les handlers
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
