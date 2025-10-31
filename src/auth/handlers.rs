use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::{create_token_pair, hash_password, verify_password, verify_refresh_token},
    config::AppState,
    middleware::AuthUser,
    repositories::UserRepository,
};

// ===== Request/Response Types =====

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// ===== Handlers =====

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Vérifier si l'email existe déjà
    if let Ok(Some(_)) = UserRepository::find_by_email(&state.db, &payload.email).await {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Email already exists".to_string(),
            }),
        ));
    }

    // Hasher le mot de passe
    let password_hash = hash_password(&payload.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to hash password".to_string(),
            }),
        )
    })?;

    // Créer l'utilisateur
    let user = UserRepository::create(&state.db, payload.email, password_hash, payload.name)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to create user: {}", e),
                }),
            )
        })?;

    // Créer les tokens JWT
    let tokens = create_token_pair(user.id, &state.jwt_config).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create tokens".to_string(),
            }),
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

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Trouver l'utilisateur par email
    let user = UserRepository::find_by_email(&state.db, &payload.email)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database error: {}", e),
                }),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid credentials".to_string(),
                }),
            )
        })?;

    // Vérifier le mot de passe
    let is_valid = verify_password(&payload.password, &user.password_hash).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to verify password".to_string(),
            }),
        )
    })?;

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid credentials".to_string(),
            }),
        ));
    }

    // Créer les tokens JWT
    let tokens = create_token_pair(user.id, &state.jwt_config).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create tokens".to_string(),
            }),
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

pub async fn logout_handler() -> Json<MessageResponse> {
    // Dans une implémentation JWT stateless, le logout est généralement géré côté client
    // en supprimant les tokens. Pour une invalidation côté serveur, vous auriez besoin
    // d'une blacklist de tokens ou d'un système de révocation.
    Json(MessageResponse {
        message: "Logged out successfully".to_string(),
    })
}

pub async fn refresh_token_handler(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Vérifier le refresh token
    let claims = verify_refresh_token(&payload.refresh_token, &state.jwt_config).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid refresh token".to_string(),
            }),
        )
    })?;

    // Récupérer l'utilisateur
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Invalid user ID in token".to_string(),
            }),
        )
    })?;

    let user = UserRepository::find_by_id(&state.db, user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database error: {}", e),
                }),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "User not found".to_string(),
                }),
            )
        })?;

    // Créer de nouveaux tokens
    let tokens = create_token_pair(user.id, &state.jwt_config).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create tokens".to_string(),
            }),
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

pub async fn me_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user = UserRepository::find_by_id(&state.db, auth_user.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database error: {}", e),
                }),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "User not found".to_string(),
                }),
            )
        })?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        created_at: user.created_at.map(|dt| dt.to_string()),
    }))
}

pub async fn update_profile_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Si l'email est fourni, vérifier qu'il n'est pas déjà utilisé
    if let Some(ref email) = payload.email {
        if let Ok(Some(existing_user)) = UserRepository::find_by_email(&state.db, email).await {
            if existing_user.id != auth_user.user_id {
                return Err((
                    StatusCode::CONFLICT,
                    Json(ErrorResponse {
                        error: "Email already in use".to_string(),
                    }),
                ));
            }
        }
    }

    let user = UserRepository::update(&state.db, auth_user.user_id, payload.name, payload.email)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to update user: {}", e),
                }),
            )
        })?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        created_at: user.created_at.map(|dt| dt.to_string()),
    }))
}

