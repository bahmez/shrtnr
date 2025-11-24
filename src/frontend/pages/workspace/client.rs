#[cfg(feature = "hydrate")]
use super::{LinkSummary, WorkspaceStats};
#[cfg(feature = "hydrate")]
use gloo_net::http::Request;

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct WorkspaceStatsResponse {
    workspace_id: String,
    total_links: u64,
    total_clicks: u64,
}

#[cfg(feature = "hydrate")]
pub async fn fetch_workspace_stats(
    workspace_id: &str,
    token: &str,
) -> Result<WorkspaceStats, String> {
    let url = format!("/api/workspaces/{}/stats", workspace_id);
    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<WorkspaceStatsResponse>()
            .await
            .map_err(|err| format!("Réponse invalide: {err}"))?;
        Ok(WorkspaceStats {
            workspace_id: parsed.workspace_id,
            total_links: parsed.total_links,
            total_clicks: parsed.total_clicks,
        })
    } else {
        let status = response.status();
        Err(format!(
            "Impossible de récupérer les statistiques (code {status})"
        ))
    }
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct LinkResponse {
    id: String,
    short_code: String,
    original_url: String,
    workspace_id: String,
    user_id: String,
    title: Option<String>,
    created_at: Option<String>,
    expires_at: Option<String>,
    is_active: bool,
    short_url: String,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct LinkListResponse {
    links: Vec<LinkResponse>,
    total: usize,
    page: u64,
    limit: u64,
}

#[cfg(feature = "hydrate")]
pub async fn fetch_recent_links(
    workspace_id: &str,
    token: &str,
    limit: u64,
) -> Result<Vec<LinkSummary>, String> {
    let url = format!("/api/links?workspace_id={}&limit={}&page=1", workspace_id, limit);
    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<LinkListResponse>()
            .await
            .map_err(|err| format!("Réponse invalide: {err}"))?;
        
        Ok(parsed
            .links
            .into_iter()
            .map(|l| LinkSummary {
                id: l.id,
                short_code: l.short_code,
                original_url: l.original_url,
                title: l.title,
                short_url: l.short_url,
                created_at: l.created_at,
                is_active: l.is_active,
            })
            .collect())
    } else {
        let status = response.status();
        Err(format!(
            "Impossible de récupérer les liens (code {status})"
        ))
    }
}

