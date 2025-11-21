use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::Redirect,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::backend::{
    config::AppState, entities::link_click, middleware::AuthUser,
    repositories::ShortenedLinkRepository,
};
use crate::shared::responses::{ApiError, ApiMessage};

#[derive(Debug, Deserialize)]
pub struct CreateLinkRequest {
    pub original_url: String,
    pub workspace_id: String,
    pub title: Option<String>,
    pub custom_code: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLinkRequest {
    pub title: Option<String>,
    pub original_url: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_limit")]
    pub limit: u64,
    pub workspace_id: Option<String>,
}

fn default_page() -> u64 {
    1
}
fn default_limit() -> u64 {
    20
}

#[derive(Debug, Serialize)]
pub struct LinkResponse {
    pub id: String,
    pub short_code: String,
    pub original_url: String,
    pub workspace_id: String,
    pub user_id: String,
    pub title: Option<String>,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
    pub is_active: bool,
    pub short_url: String,
}

#[derive(Debug, Serialize)]
pub struct LinkListResponse {
    pub links: Vec<LinkResponse>,
    pub total: usize,
    pub page: u64,
    pub limit: u64,
}

#[axum::debug_handler]
pub async fn create_link_handler(
    auth_user: AuthUser,
    Json(payload): Json<CreateLinkRequest>,
) -> Result<Json<LinkResponse>, (StatusCode, Json<ApiError>)> {
    let state = &auth_user.state;
    if !is_valid_url(&payload.original_url) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new(
                "Invalid URL. Must start with http:// or https://",
            )),
        ));
    }

    let workspace_id = Uuid::parse_str(&payload.workspace_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Invalid workspace ID")),
        )
    })?;

    let short_code = if let Some(custom) = payload.custom_code {
        if !is_valid_short_code(&custom) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError::new(
                    "Invalid custom code. Use 3-20 alphanumeric characters, hyphens, or underscores",
                )),
            ));
        }

        if let Ok(Some(_)) = ShortenedLinkRepository::find_by_short_code(&state.db, &custom).await {
            return Err((
                StatusCode::CONFLICT,
                Json(ApiError::new("This short code is already taken")),
            ));
        }

        custom
    } else {
        generate_short_code(&state.db).await?
    };

    let expires_at = if let Some(exp_str) = payload.expires_at {
        Some(
            chrono::DateTime::parse_from_rfc3339(&exp_str)
                .map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(ApiError::new(
                            "Invalid expiration date format. Use ISO 8601 (e.g., 2024-12-31T23:59:59Z)",
                        )),
                    )
                })?
                .naive_utc(),
        )
    } else {
        None
    };

    let link = ShortenedLinkRepository::create(
        &state.db,
        short_code,
        payload.original_url,
        workspace_id,
        auth_user.user_id,
        payload.title,
        expires_at,
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new(format!("Failed to create link: {}", e))),
        )
    })?;

    Ok(Json(link_to_response(link)))
}

pub async fn list_links_handler(
    auth_user: AuthUser,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<LinkListResponse>, (StatusCode, Json<ApiError>)> {
    let state = &auth_user.state;
    let offset = (params.page - 1) * params.limit;

    let links = if let Some(workspace_id_str) = params.workspace_id {
        let workspace_id = Uuid::parse_str(&workspace_id_str).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiError::new("Invalid workspace ID")),
            )
        })?;

        ShortenedLinkRepository::find_by_workspace(&state.db, workspace_id, params.limit, offset)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError::new(format!("Database error: {}", e))),
                )
            })?
    } else {
        ShortenedLinkRepository::find_by_user(&state.db, auth_user.user_id, params.limit, offset)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError::new(format!("Database error: {}", e))),
                )
            })?
    };

    let total = links.len();
    let link_responses: Vec<LinkResponse> = links.into_iter().map(link_to_response).collect();

    Ok(Json(LinkListResponse {
        links: link_responses,
        total,
        page: params.page,
        limit: params.limit,
    }))
}

pub async fn get_link_handler(
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<LinkResponse>, (StatusCode, Json<ApiError>)> {
    let state = &auth_user.state;
    let link_id = Uuid::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Invalid link ID")),
        )
    })?;

    let link = ShortenedLinkRepository::find_by_id(&state.db, link_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new(format!("Database error: {}", e))),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(ApiError::new("Link not found"))))?;

    if link.user_id != auth_user.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ApiError::new(
                "You don't have permission to access this link",
            )),
        ));
    }

    Ok(Json(link_to_response(link)))
}

pub async fn update_link_handler(
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdateLinkRequest>,
) -> Result<Json<LinkResponse>, (StatusCode, Json<ApiError>)> {
    let state = &auth_user.state;
    let link_id = Uuid::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Invalid link ID")),
        )
    })?;

    let existing_link = ShortenedLinkRepository::find_by_id(&state.db, link_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new(format!("Database error: {}", e))),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(ApiError::new("Link not found"))))?;

    if existing_link.user_id != auth_user.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ApiError::new(
                "You don't have permission to update this link",
            )),
        ));
    }

    if let Some(ref url) = payload.original_url {
        if !is_valid_url(url) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError::new(
                    "Invalid URL. Must start with http:// or https://",
                )),
            ));
        }
    }

    let link = ShortenedLinkRepository::update(
        &state.db,
        link_id,
        payload.title,
        payload.original_url,
        payload.is_active,
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::new(format!("Failed to update link: {}", e))),
        )
    })?;

    state.link_cache.invalidate(&link.short_code).await;

    Ok(Json(link_to_response(link)))
}

pub async fn delete_link_handler(
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiMessage>, (StatusCode, Json<ApiError>)> {
    let state = &auth_user.state;
    let link_id = Uuid::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Invalid link ID")),
        )
    })?;

    let existing_link = ShortenedLinkRepository::find_by_id(&state.db, link_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new(format!("Database error: {}", e))),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(ApiError::new("Link not found"))))?;

    if existing_link.user_id != auth_user.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ApiError::new(
                "You don't have permission to delete this link",
            )),
        ));
    }

    let short_code = existing_link.short_code.clone();

    ShortenedLinkRepository::delete(&state.db, link_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new(format!("Failed to delete link: {}", e))),
            )
        })?;

    state.link_cache.invalidate(&short_code).await;

    Ok(Json(ApiMessage::new("Link deleted successfully")))
}

pub async fn redirect_handler(
    Extension(state): Extension<AppState>,
    Path(short_code): Path<String>,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<std::net::SocketAddr>,
    headers: axum::http::HeaderMap,
) -> Result<Redirect, (StatusCode, Json<ApiError>)> {
    let link = if let Some(cached_link) = state.link_cache.get(&short_code).await {
        cached_link
    } else {
        let db_link = ShortenedLinkRepository::find_by_short_code(&state.db, &short_code)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError::new(format!("Database error: {}", e))),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiError::new("Short link not found")),
                )
            })?;

        if db_link.is_active {
            if let Some(expires_at) = db_link.expires_at {
                if expires_at >= chrono::Utc::now().naive_utc() {
                    state
                        .link_cache
                        .insert(short_code.clone(), db_link.clone())
                        .await;
                }
            } else {
                state
                    .link_cache
                    .insert(short_code.clone(), db_link.clone())
                    .await;
            }
        }

        db_link
    };

    if !link.is_active {
        return Err((
            StatusCode::GONE,
            Json(ApiError::new("This link has been deactivated")),
        ));
    }

    if let Some(expires_at) = link.expires_at {
        if expires_at < chrono::Utc::now().naive_utc() {
            return Err((
                StatusCode::GONE,
                Json(ApiError::new("This link has expired")),
            ));
        }
    }

    let db_clone = state.db.clone();
    let link_id = link.id;
    tokio::spawn(async move {
        let _ = record_click(
            &db_clone,
            link_id,
            addr.ip().to_string(),
            headers
                .get("user-agent")
                .and_then(|h| h.to_str().ok())
                .map(String::from),
            headers
                .get("referer")
                .and_then(|h| h.to_str().ok())
                .map(String::from),
        )
        .await;
    });

    Ok(Redirect::temporary(&link.original_url))
}

fn link_to_response(link: crate::backend::entities::shortened_link::Model) -> LinkResponse {
    LinkResponse {
        id: link.id.to_string(),
        short_code: link.short_code.clone(),
        original_url: link.original_url,
        workspace_id: link.workspace_id.to_string(),
        user_id: link.user_id.to_string(),
        title: link.title,
        created_at: link.created_at.map(|dt| dt.to_string()),
        expires_at: link.expires_at.map(|dt| dt.to_string()),
        is_active: link.is_active,
        short_url: format!("http://localhost:3000/{}", link.short_code),
    }
}

fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

fn is_valid_short_code(code: &str) -> bool {
    code.len() >= 3
        && code.len() <= 20
        && code
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

async fn generate_short_code(
    db: &sea_orm::DatabaseConnection,
) -> Result<String, (StatusCode, Json<ApiError>)> {
    use rand::Rng;

    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const CODE_LENGTH: usize = 6;

    for _ in 0..10 {
        // Generate code in a separate scope so rng is dropped before await
        let code: String = {
            let mut rng = rand::thread_rng();
            (0..CODE_LENGTH)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect()
        };

        if let Ok(None) = ShortenedLinkRepository::find_by_short_code(db, &code).await {
            return Ok(code);
        }
    }

    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError::new(
            "Failed to generate unique short code. Please try again.",
        )),
    ))
}

async fn record_click(
    db: &sea_orm::DatabaseConnection,
    link_id: Uuid,
    ip_address: String,
    user_agent: Option<String>,
    referer: Option<String>,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::*;

    let click = link_click::ActiveModel {
        id: Set(Uuid::new_v4()),
        link_id: Set(link_id),
        clicked_at: Set(Some(chrono::Utc::now().naive_utc())),
        ip_address: Set(Some(ip_address)),
        user_agent: Set(user_agent),
        referer: Set(referer),
        country: Set(None),
        city: Set(None),
    };

    click.insert(db).await?;
    Ok(())
}
