use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    middleware::AuthUser,
    repositories::{
        LinkClickRepository,
        ShortenedLinkRepository,
        WorkspaceMemberRepository,
        WorkspaceRepository,
    },
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse { pub error: String }

// ===== Link Stats =====

#[derive(Debug, Serialize)]
pub struct LinkStatsResponse {
    pub link_id: String,
    pub total_clicks: u64,
}

pub async fn get_link_stats_handler(
    auth_user: AuthUser,
    Path(link_id): Path<String>,
) -> Result<Json<LinkStatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let link_uuid = Uuid::parse_str(&link_id).map_err(|_| (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse { error: "id invalide (UUID attendu)".to_string() })
    ))?;

    let link = ShortenedLinkRepository::find_by_id(&auth_user.state.db, link_uuid)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or((StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Lien non trouvé".to_string() })))?;

    let ws = WorkspaceRepository::find_by_id(&auth_user.state.db, link.workspace_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or((StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Workspace non trouvé".to_string() })))?;
    if ws.owner_id != auth_user.user_id {
        let me = WorkspaceMemberRepository::find_by_ids(&auth_user.state.db, ws.id, auth_user.user_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;
        if me.is_none() {
            return Err((StatusCode::FORBIDDEN, Json(ErrorResponse { error: "Accès interdit".to_string() })));
        }
    }

    let total = LinkClickRepository::count_by_link(&auth_user.state.db, link_uuid)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(Json(LinkStatsResponse { link_id, total_clicks: total }))
}

// ===== Workspace Stats =====

#[derive(Debug, Serialize)]
pub struct WorkspaceStatsResponse {
    pub workspace_id: String,
    pub total_links: u64,
    pub total_clicks: u64,
}

pub async fn get_workspace_stats_handler(
    auth_user: AuthUser,
    Path(workspace_id): Path<String>,
) -> Result<Json<WorkspaceStatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ws_id = Uuid::parse_str(&workspace_id).map_err(|_| (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse { error: "id invalide (UUID attendu)".to_string() })
    ))?;

    let ws = WorkspaceRepository::find_by_id(&auth_user.state.db, ws_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
        .ok_or((StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Workspace non trouvé".to_string() })))?;

    if ws.owner_id != auth_user.user_id {
        let me = WorkspaceMemberRepository::find_by_ids(&auth_user.state.db, ws.id, auth_user.user_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;
        if me.is_none() {
            return Err((StatusCode::FORBIDDEN, Json(ErrorResponse { error: "Accès interdit".to_string() })));
        }
    }

    let links = ShortenedLinkRepository::find_by_workspace(&auth_user.state.db, ws_id, 10_000, 0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    let total_links = links.len() as u64;
    let mut total_clicks: u64 = 0;
    for l in links {
        let c = LinkClickRepository::count_by_link(&auth_user.state.db, l.id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;
        total_clicks += c;
    }

    Ok(Json(WorkspaceStatsResponse { workspace_id, total_links, total_clicks }))
}

// ===== Dashboard Stats =====

#[derive(Debug, Serialize)]
pub struct DashboardStatsResponse {
    pub total_workspaces: u64,
    pub total_links: u64,
    pub total_clicks: u64,
}

pub async fn get_dashboard_stats_handler(
    auth_user: AuthUser,
) -> Result<Json<DashboardStatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let workspaces = WorkspaceRepository::find_by_owner(&auth_user.state.db, auth_user.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    let mut total_links: u64 = 0;
    let mut total_clicks: u64 = 0;
    for ws in workspaces {
        let links = ShortenedLinkRepository::find_by_workspace(&auth_user.state.db, ws.id, 10_000, 0)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;
        total_links += links.len() as u64;
        for l in links {
            let c = LinkClickRepository::count_by_link(&auth_user.state.db, l.id)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;
            total_clicks += c;
        }
    }

    Ok(Json(DashboardStatsResponse {
        total_workspaces: WorkspaceRepository::find_by_owner(&auth_user.state.db, auth_user.user_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?
            .len() as u64,
        total_links,
        total_clicks,
    }))
}


