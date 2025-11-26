#[cfg(feature = "hydrate")]
use super::LinkItem;
#[cfg(feature = "hydrate")]
use gloo_net::http::Request;

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
#[derive(serde::Deserialize)]
struct ApiErrorResponse {
    error: String,
}

#[cfg(feature = "hydrate")]
#[derive(Clone, Debug)]
pub struct LinkListData {
    pub links: Vec<LinkItem>,
    pub total: usize,
    pub page: u64,
    pub limit: u64,
}

#[cfg(feature = "hydrate")]
#[derive(Clone, Debug)]
pub struct LinkDetail {
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

#[cfg(feature = "hydrate")]
#[derive(serde::Serialize)]
struct CreateLinkPayload {
    original_url: String,
    workspace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<String>,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Serialize)]
struct UpdateLinkPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_active: Option<bool>,
}

#[cfg(feature = "hydrate")]
pub async fn fetch_links(
    workspace_id: &str,
    token: &str,
    page: u64,
    limit: u64,
) -> Result<LinkListData, String> {
    let url = format!(
        "/api/links?workspace_id={}&page={}&limit={}",
        workspace_id, page, limit
    );
    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<LinkListResponse>()
            .await
            .map_err(|err| format!("Reponse invalide: {err}"))?;

        Ok(LinkListData {
            links: parsed
                .links
                .into_iter()
                .map(|l| LinkItem {
                    id: l.id,
                    short_code: l.short_code,
                    original_url: l.original_url,
                    workspace_id: l.workspace_id,
                    user_id: l.user_id,
                    title: l.title,
                    created_at: l.created_at,
                    expires_at: l.expires_at,
                    is_active: l.is_active,
                    short_url: l.short_url,
                })
                .collect(),
            total: parsed.total,
            page: parsed.page,
            limit: parsed.limit,
        })
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Impossible de recuperer les liens (code {status})"));
        Err(message)
    }
}

#[cfg(feature = "hydrate")]
pub async fn create_link(
    token: &str,
    original_url: String,
    workspace_id: String,
    title: Option<String>,
    custom_code: Option<String>,
    expires_at: Option<String>,
) -> Result<LinkDetail, String> {
    let payload = CreateLinkPayload {
        original_url,
        workspace_id,
        title,
        custom_code,
        expires_at,
    };

    let response = Request::post("/api/links")
        .header("Authorization", &format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<LinkResponse>()
            .await
            .map_err(|err| format!("Reponse invalide: {err}"))?;

        Ok(LinkDetail {
            id: parsed.id,
            short_code: parsed.short_code,
            original_url: parsed.original_url,
            workspace_id: parsed.workspace_id,
            user_id: parsed.user_id,
            title: parsed.title,
            created_at: parsed.created_at,
            expires_at: parsed.expires_at,
            is_active: parsed.is_active,
            short_url: parsed.short_url,
        })
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Impossible de creer le lien (code {status})"));
        Err(message)
    }
}

#[cfg(feature = "hydrate")]
pub async fn update_link(
    token: &str,
    link_id: &str,
    title: Option<String>,
    original_url: Option<String>,
    is_active: Option<bool>,
) -> Result<LinkDetail, String> {
    let payload = UpdateLinkPayload {
        title,
        original_url,
        is_active,
    };

    let url = format!("/api/links/{}", link_id);
    let response = Request::put(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<LinkResponse>()
            .await
            .map_err(|err| format!("Reponse invalide: {err}"))?;

        Ok(LinkDetail {
            id: parsed.id,
            short_code: parsed.short_code,
            original_url: parsed.original_url,
            workspace_id: parsed.workspace_id,
            user_id: parsed.user_id,
            title: parsed.title,
            created_at: parsed.created_at,
            expires_at: parsed.expires_at,
            is_active: parsed.is_active,
            short_url: parsed.short_url,
        })
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Impossible de modifier le lien (code {status})"));
        Err(message)
    }
}

#[cfg(feature = "hydrate")]
pub async fn delete_link(token: &str, link_id: &str) -> Result<(), String> {
    let url = format!("/api/links/{}", link_id);
    let response = Request::delete(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        Ok(())
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Impossible de supprimer le lien (code {status})"));
        Err(message)
    }
}
