//! Module client pour les appels API des paramètres.
//!
//! Fournit les fonctions pour interagir avec l'API de gestion des workspaces
//! et membres côté client (WASM).
//!
//! # Note
//!
//! Ce module est uniquement disponible avec la feature `hydrate`.

#[cfg(feature = "hydrate")]
use gloo_net::http::Request;

#[cfg(feature = "hydrate")]
#[derive(serde::Serialize)]
struct UpdateWorkspacePayload {
    name: Option<String>,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct WorkspaceResponse {
    id: String,
    name: String,
    owner_id: String,
    #[allow(dead_code)]
    created_at: Option<String>,
    #[allow(dead_code)]
    updated_at: Option<String>,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct UpdateWorkspaceResponse {
    workspace: WorkspaceResponse,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct ApiErrorResponse {
    error: String,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Serialize)]
struct AddWorkspaceMemberPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    role: String,
}

/// Réponse représentant un membre de workspace.
#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
pub struct WorkspaceMemberResponse {
    pub workspace_id: String,
    pub user_id: String,
    pub role: String,
    pub joined_at: Option<String>,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct AddWorkspaceMemberResponse {
    member: WorkspaceMemberResponse,
}

/// Réponse représentant un membre de workspace avec les informations utilisateur.
#[cfg(feature = "hydrate")]
#[derive(Clone, serde::Deserialize)]
pub struct WorkspaceMemberWithUserResponse {
    pub workspace_id: String,
    pub user_id: String,
    pub user_email: String,
    pub user_name: Option<String>,
    pub role: String,
    pub joined_at: Option<String>,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Deserialize)]
struct ListWorkspaceMembersResponse {
    members: Vec<WorkspaceMemberWithUserResponse>,
}

/// Met à jour un workspace.
///
/// # Arguments
///
/// * `workspace_id` - ID du workspace
/// * `token` - Token d'accès JWT
/// * `name` - Nouveau nom (optionnel)
///
/// # Returns
///
/// * `Ok(WorkspaceSummary)` - Workspace mis à jour
/// * `Err(String)` - En cas d'erreur
#[cfg(feature = "hydrate")]
pub async fn update_workspace(
    workspace_id: &str,
    token: &str,
    name: Option<String>,
) -> Result<crate::frontend::state::WorkspaceSummary, String> {
    let url = format!("/api/workspaces/{}", workspace_id);
    let payload = UpdateWorkspacePayload { name };
    
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
            .json::<UpdateWorkspaceResponse>()
            .await
            .map_err(|err| format!("Réponse invalide: {err}"))?;
        
        Ok(crate::frontend::state::WorkspaceSummary {
            id: parsed.workspace.id,
            name: parsed.workspace.name,
            owner_id: parsed.workspace.owner_id,
        })
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Une erreur est survenue (code {status})"));
        Err(message)
    }
}

/// Ajoute un membre à un workspace.
///
/// # Arguments
///
/// * `workspace_id` - ID du workspace
/// * `token` - Token d'accès JWT
/// * `email` - Email du membre (optionnel, si user_id n'est pas fourni)
/// * `user_id` - ID de l'utilisateur (optionnel, si email n'est pas fourni)
/// * `role` - Rôle du membre ("owner", "admin", "member")
///
/// # Returns
///
/// * `Ok(WorkspaceMemberResponse)` - Membre ajouté
/// * `Err(String)` - En cas d'erreur
#[cfg(feature = "hydrate")]
pub async fn add_workspace_member(
    workspace_id: &str,
    token: &str,
    email: Option<&str>,
    user_id: Option<&str>,
    role: &str,
) -> Result<WorkspaceMemberResponse, String> {
    let url = format!("/api/workspaces/{}/members", workspace_id);
    let payload = AddWorkspaceMemberPayload {
        user_id: user_id.map(|s| s.to_string()),
        email: email.map(|s| s.to_string()),
        role: role.to_string(),
    };
    
    let response = Request::post(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<AddWorkspaceMemberResponse>()
            .await
            .map_err(|err| format!("Réponse invalide: {err}"))?;
        
        Ok(parsed.member)
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Une erreur est survenue (code {status})"));
        Err(message)
    }
}

/// Liste tous les membres d'un workspace.
///
/// # Arguments
///
/// * `workspace_id` - ID du workspace
/// * `token` - Token d'accès JWT
///
/// # Returns
///
/// * `Ok(Vec<WorkspaceMemberWithUserResponse>)` - Liste des membres
/// * `Err(String)` - En cas d'erreur
#[cfg(feature = "hydrate")]
pub async fn list_workspace_members(
    workspace_id: &str,
    token: &str,
) -> Result<Vec<WorkspaceMemberWithUserResponse>, String> {
    let url = format!("/api/workspaces/{}/members", workspace_id);

    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<ListWorkspaceMembersResponse>()
            .await
            .map_err(|err| format!("Réponse invalide: {err}"))?;

        Ok(parsed.members)
    } else {
        let status = response.status();
        let message = response
            .json::<ApiErrorResponse>()
            .await
            .map(|err| err.error)
            .unwrap_or_else(|_| format!("Une erreur est survenue (code {status})"));
        Err(message)
    }
}

/// Retire un membre d'un workspace.
///
/// # Arguments
///
/// * `workspace_id` - ID du workspace
/// * `token` - Token d'accès JWT
/// * `user_id` - ID de l'utilisateur à retirer
///
/// # Returns
///
/// * `Ok(())` - Membre retiré avec succès
/// * `Err(String)` - En cas d'erreur
#[cfg(feature = "hydrate")]
pub async fn remove_workspace_member(
    workspace_id: &str,
    token: &str,
    user_id: &str,
) -> Result<(), String> {
    let url = format!("/api/workspaces/{}/members/{}", workspace_id, user_id);

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
            .unwrap_or_else(|_| format!("Une erreur est survenue (code {status})"));
        Err(message)
    }
}
