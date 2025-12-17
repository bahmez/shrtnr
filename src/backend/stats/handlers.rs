use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::backend::{
    middleware::AuthUser,
    repositories::{
        LinkClickRepository, ShortenedLinkRepository, WorkspaceMemberRepository,
        WorkspaceRepository,
    },
};
use crate::shared::responses::ApiError;

pub type ErrorResponse = ApiError;

// ===== Link Stats =====

#[derive(Debug, Serialize)]
pub struct LinkStatsResponse {
    pub link_id: String,
    pub total_clicks: u64,
}

/// Handler pour obtenir les statistiques d'un lien spécifique.
///
/// Retourne le nombre total de clics pour un lien donné.
///
/// # Endpoint
///
/// `GET /api/links/{link_id}/stats`
///
/// # Arguments
///
/// * `auth_user` - Utilisateur authentifié
/// * `link_id` - UUID du lien
///
/// # Returns
///
/// Statistiques du lien (total de clics).
///
/// # Errors
///
/// * `400 Bad Request` - link_id invalide
/// * `401 Unauthorized` - Token invalide
/// * `403 Forbidden` - L'utilisateur n'a pas accès au workspace du lien
/// * `404 Not Found` - Lien ou workspace introuvable
/// * `500 Internal Server Error` - Erreur de base de données
pub async fn get_link_stats_handler(
    auth_user: AuthUser,
    Path(link_id): Path<String>,
) -> Result<Json<LinkStatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let link_uuid = Uuid::parse_str(&link_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    let link = ShortenedLinkRepository::find_by_id(&auth_user.state.db, link_uuid)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Lien non trouvé".to_string(),
            }),
        ))?;

    let ws = WorkspaceRepository::find_by_id(&auth_user.state.db, link.workspace_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Workspace non trouvé".to_string(),
            }),
        ))?;
    if ws.owner_id != auth_user.user_id {
        let me =
            WorkspaceMemberRepository::find_by_ids(&auth_user.state.db, ws.id, auth_user.user_id)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: e.to_string(),
                        }),
                    )
                })?;
        if me.is_none() {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "Accès interdit".to_string(),
                }),
            ));
        }
    }

    let total = LinkClickRepository::count_by_link(&auth_user.state.db, link_uuid)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    Ok(Json(LinkStatsResponse {
        link_id,
        total_clicks: total,
    }))
}

// ===== Workspace Stats =====

#[derive(Debug, Serialize)]
pub struct WorkspaceStatsResponse {
    pub workspace_id: String,
    pub total_links: u64,
    pub total_clicks: u64,
}

/// Handler pour obtenir les statistiques d'un workspace.
///
/// Retourne le nombre total de liens et le nombre total de clics pour un workspace.
///
/// # Endpoint
///
/// `GET /api/workspaces/{workspace_id}/stats`
///
/// # Arguments
///
/// * `auth_user` - Utilisateur authentifié
/// * `workspace_id` - UUID du workspace
///
/// # Returns
///
/// Statistiques du workspace (total_links, total_clicks).
///
/// # Errors
///
/// * `400 Bad Request` - workspace_id invalide
/// * `401 Unauthorized` - Token invalide
/// * `403 Forbidden` - L'utilisateur n'a pas accès au workspace
/// * `404 Not Found` - Workspace introuvable
/// * `500 Internal Server Error` - Erreur de base de données
pub async fn get_workspace_stats_handler(
    auth_user: AuthUser,
    Path(workspace_id): Path<String>,
) -> Result<Json<WorkspaceStatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ws_id = Uuid::parse_str(&workspace_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    let ws = WorkspaceRepository::find_by_id(&auth_user.state.db, ws_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Workspace non trouvé".to_string(),
            }),
        ))?;

    if ws.owner_id != auth_user.user_id {
        let me =
            WorkspaceMemberRepository::find_by_ids(&auth_user.state.db, ws.id, auth_user.user_id)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: e.to_string(),
                        }),
                    )
                })?;
        if me.is_none() {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "Accès interdit".to_string(),
                }),
            ));
        }
    }

    let links = ShortenedLinkRepository::find_by_workspace(&auth_user.state.db, ws_id, 10_000, 0)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let total_links = links.len() as u64;
    let mut total_clicks: u64 = 0;
    for l in links {
        let c = LinkClickRepository::count_by_link(&auth_user.state.db, l.id)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: e.to_string(),
                    }),
                )
            })?;
        total_clicks += c;
    }

    Ok(Json(WorkspaceStatsResponse {
        workspace_id,
        total_links,
        total_clicks,
    }))
}

// ===== Dashboard Stats =====

#[derive(Debug, Serialize)]
pub struct DashboardStatsResponse {
    pub total_workspaces: u64,
    pub total_links: u64,
    pub total_clicks: u64,
}

/// Handler pour obtenir les statistiques globales du dashboard.
///
/// Retourne les statistiques agrégées de tous les workspaces de l'utilisateur :
/// nombre total de workspaces, de liens, et de clics.
///
/// # Endpoint
///
/// `GET /api/stats/dashboard`
///
/// # Arguments
///
/// * `auth_user` - Utilisateur authentifié
///
/// # Returns
///
/// Statistiques globales du dashboard (total_workspaces, total_links, total_clicks).
///
/// # Errors
///
/// * `401 Unauthorized` - Token invalide
/// * `500 Internal Server Error` - Erreur de base de données
pub async fn get_dashboard_stats_handler(
    auth_user: AuthUser,
) -> Result<Json<DashboardStatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let workspaces = WorkspaceRepository::find_by_owner(&auth_user.state.db, auth_user.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let mut total_links: u64 = 0;
    let mut total_clicks: u64 = 0;
    for ws in workspaces {
        let links =
            ShortenedLinkRepository::find_by_workspace(&auth_user.state.db, ws.id, 10_000, 0)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: e.to_string(),
                        }),
                    )
                })?;
        total_links += links.len() as u64;
        for l in links {
            let c = LinkClickRepository::count_by_link(&auth_user.state.db, l.id)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: e.to_string(),
                        }),
                    )
                })?;
            total_clicks += c;
        }
    }

    Ok(Json(DashboardStatsResponse {
        total_workspaces: WorkspaceRepository::find_by_owner(&auth_user.state.db, auth_user.user_id)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: e.to_string(),
                    }),
                )
            })?
            .len() as u64,
        total_links,
        total_clicks,
    }))
}
