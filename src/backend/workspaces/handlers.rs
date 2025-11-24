use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::backend::{
    entities::workspace_member::Role,
    middleware::AuthUser,
    repositories::{UserRepository, WorkspaceMemberRepository, WorkspaceRepository},
};
use crate::shared::responses::ApiError;

// ===== Request/Response Types =====

#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub owner_id: String,
}

#[derive(Debug, Serialize)]
pub struct WorkspaceResponse {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListWorkspacesRequest {
    pub owner_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListWorkspacesResponse {
    pub workspaces: Vec<WorkspaceResponse>,
}

#[derive(Debug, Deserialize)]
pub struct GetWorkspaceRequest {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct GetWorkspaceResponse {
    pub workspace: WorkspaceResponse,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateWorkspaceResponse {
    pub workspace: WorkspaceResponse,
}

#[derive(Debug, Deserialize)]
pub struct DeleteWorkspaceRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteWorkspaceResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct AddWorkspaceMemberRequest {
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    pub role: Role,
}

#[derive(Debug, Serialize)]
pub struct AddWorkspaceMemberResponse {
    pub member: WorkspaceMemberResponse,
}

#[derive(Debug, Deserialize)]
pub struct RemoveWorkspaceMemberRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize)]
pub struct RemoveWorkspaceMemberResponse {
    pub message: String,
}

pub type ErrorResponse = ApiError;

#[derive(Debug, Serialize)]
pub struct WorkspaceMemberResponse {
    pub workspace_id: String,
    pub user_id: String,
    pub role: Role,
    pub joined_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WorkspaceMemberWithUserResponse {
    pub workspace_id: String,
    pub user_id: String,
    pub user_email: String,
    pub user_name: Option<String>,
    pub role: String,
    pub joined_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListWorkspaceMembersResponse {
    pub members: Vec<WorkspaceMemberWithUserResponse>,
}

// ===== Handlers =====

pub async fn create_workspace_handler(
    auth_user: AuthUser,
    Json(payload): Json<CreateWorkspaceRequest>,
) -> Result<Json<WorkspaceResponse>, (StatusCode, Json<ErrorResponse>)> {
    let owner_uuid = uuid::Uuid::parse_str(&payload.owner_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "owner_id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    if owner_uuid != auth_user.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "owner_id ne correspond pas à l'utilisateur authentifié".to_string(),
            }),
        ));
    }

    let created = WorkspaceRepository::create(&auth_user.state.db, payload.name, owner_uuid)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Erreur lors de la création du workspace: {}", e),
                }),
            )
        })?;

    let response = WorkspaceResponse {
        id: created.id.to_string(),
        name: created.name,
        owner_id: created.owner_id.to_string(),
        created_at: created.created_at.map(|dt| dt.to_string()),
        updated_at: created.updated_at.map(|dt| dt.to_string()),
    };

    Ok(Json(response))
}

pub async fn list_workspaces_handler(
    auth_user: AuthUser,
    Query(params): Query<ListWorkspacesRequest>,
) -> Result<Json<ListWorkspacesResponse>, (StatusCode, Json<ErrorResponse>)> {
    let owner_uuid = if let Some(owner) = params.owner_id.as_deref() {
        let parsed = Uuid::parse_str(owner).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "owner_id invalide (UUID attendu)".to_string(),
                }),
            )
        })?;
        if parsed != auth_user.user_id {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "Accès interdit à ces workspaces".to_string(),
                }),
            ));
        }
        parsed
    } else {
        auth_user.user_id
    };

    let owned_workspaces = WorkspaceRepository::find_by_owner(&auth_user.state.db, owner_uuid)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Erreur lors de la liste des workspaces: {}", e),
                }),
            )
        })?;

    let memberships = WorkspaceMemberRepository::find_by_user(&auth_user.state.db, auth_user.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Erreur lors de la récupération des membreships: {}", e),
                }),
            )
        })?;

    // Récupérer les workspaces correspondants aux membreships
    let mut member_workspaces = Vec::new();
    for membership in memberships {
        if let Some(workspace) = WorkspaceRepository::find_by_id(&auth_user.state.db, membership.workspace_id)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Erreur lors de la récupération du workspace: {}", e),
                    }),
                )
            })?
        {
            member_workspaces.push(workspace);
        }
    }

    // Combiner les deux listes en évitant les doublons
    let mut all_workspaces = owned_workspaces;
    for member_ws in member_workspaces {
        if !all_workspaces.iter().any(|ws| ws.id == member_ws.id) {
            all_workspaces.push(member_ws);
        }
    }

    let workspaces = all_workspaces
        .into_iter()
        .map(|w| WorkspaceResponse {
            id: w.id.to_string(),
            name: w.name,
            owner_id: w.owner_id.to_string(),
            created_at: w.created_at.map(|dt| dt.to_string()),
            updated_at: w.updated_at.map(|dt| dt.to_string()),
        })
        .collect();

    Ok(Json(ListWorkspacesResponse { workspaces }))
}

pub async fn get_workspace_handler(
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<GetWorkspaceResponse>, (StatusCode, Json<ErrorResponse>)> {
    let workspace_id = Uuid::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    let found = WorkspaceRepository::find_by_id(&auth_user.state.db, workspace_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Erreur lors de la récupération du workspace: {}", e),
                }),
            )
        })?;

    let model = found.ok_or((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "Workspace non trouvé".to_string(),
        }),
    ))?;

    if model.owner_id != auth_user.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Accès interdit à ce workspace".to_string(),
            }),
        ));
    }

    let response = WorkspaceResponse {
        id: model.id.to_string(),
        name: model.name,
        owner_id: model.owner_id.to_string(),
        created_at: model.created_at.map(|dt| dt.to_string()),
        updated_at: model.updated_at.map(|dt| dt.to_string()),
    };

    Ok(Json(GetWorkspaceResponse {
        workspace: response,
    }))
}

pub async fn update_workspace_handler(
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdateWorkspaceRequest>,
) -> Result<Json<UpdateWorkspaceResponse>, (StatusCode, Json<ErrorResponse>)> {
    let workspace_id = Uuid::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    let existing = WorkspaceRepository::find_by_id(&auth_user.state.db, workspace_id)
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

    if existing.owner_id != auth_user.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Accès interdit".to_string(),
            }),
        ));
    }

    let updated = WorkspaceRepository::update(&auth_user.state.db, workspace_id, payload.name)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let response = WorkspaceResponse {
        id: updated.id.to_string(),
        name: updated.name,
        owner_id: updated.owner_id.to_string(),
        created_at: updated.created_at.map(|dt| dt.to_string()),
        updated_at: updated.updated_at.map(|dt| dt.to_string()),
    };

    Ok(Json(UpdateWorkspaceResponse {
        workspace: response,
    }))
}

pub async fn delete_workspace_handler(
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<DeleteWorkspaceResponse>, (StatusCode, Json<ErrorResponse>)> {
    let workspace_id = Uuid::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    let existing = WorkspaceRepository::find_by_id(&auth_user.state.db, workspace_id)
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

    if existing.owner_id != auth_user.user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Accès interdit".to_string(),
            }),
        ));
    }

    WorkspaceRepository::delete(&auth_user.state.db, workspace_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    Ok(Json(DeleteWorkspaceResponse {
        message: "Workspace supprimé".to_string(),
    }))
}

pub async fn add_workspace_member_handler(
    auth_user: AuthUser,
    Path(workspace_id): Path<String>,
    Json(payload): Json<AddWorkspaceMemberRequest>,
) -> Result<Json<AddWorkspaceMemberResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ws_id = Uuid::parse_str(&workspace_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "workspace_id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;
    
    let user_id = if let Some(user_id_str) = &payload.user_id {
        Uuid::parse_str(user_id_str).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "user_id invalide (UUID attendu)".to_string(),
                }),
            )
        })?
    } else if let Some(email) = &payload.email {
        let user = crate::backend::repositories::UserRepository::find_by_email(&auth_user.state.db, email)
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
                    error: format!("Aucun utilisateur trouvé avec l'email {}", email),
                }),
            ))?;
        user.id
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "user_id ou email requis".to_string(),
            }),
        ));
    };

    let existing = WorkspaceRepository::find_by_id(&auth_user.state.db, ws_id)
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
    if existing.owner_id != auth_user.user_id {
        let me =
            WorkspaceMemberRepository::find_by_ids(&auth_user.state.db, ws_id, auth_user.user_id)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: e.to_string(),
                        }),
                    )
                })?;
        let is_admin = matches!(
            me.as_ref().map(|m| &m.role),
            Some(Role::Admin) | Some(Role::Owner)
        );
        if !is_admin {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "Accès interdit".to_string(),
                }),
            ));
        }
    }

    if matches!(payload.role, Role::Owner) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Rôle 'owner' réservé, utilisez 'admin' ou 'member'".to_string(),
            }),
        ));
    }

    if WorkspaceMemberRepository::find_by_ids(&auth_user.state.db, ws_id, user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?
        .is_some()
    {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Membre déjà présent".to_string(),
            }),
        ));
    }

    let created =
        WorkspaceMemberRepository::create(&auth_user.state.db, ws_id, user_id, payload.role)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: e.to_string(),
                    }),
                )
            })?;

    let resp = WorkspaceMemberResponse {
        workspace_id: created.workspace_id.to_string(),
        user_id: created.user_id.to_string(),
        role: created.role,
        joined_at: created.joined_at.map(|dt| dt.to_string()),
    };

    Ok(Json(AddWorkspaceMemberResponse { member: resp }))
}

pub async fn remove_workspace_member_handler(
    auth_user: AuthUser,
    Path((workspace_id, user_id)): Path<(String, String)>,
) -> Result<Json<RemoveWorkspaceMemberResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ws_id = Uuid::parse_str(&workspace_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "workspace_id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;
    let user_id = Uuid::parse_str(&user_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "user_id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    let existing = WorkspaceRepository::find_by_id(&auth_user.state.db, ws_id)
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

    if existing.owner_id != auth_user.user_id {
        let me =
            WorkspaceMemberRepository::find_by_ids(&auth_user.state.db, ws_id, auth_user.user_id)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: e.to_string(),
                        }),
                    )
                })?;
        let is_admin = matches!(
            me.as_ref().map(|m| &m.role),
            Some(Role::Admin) | Some(Role::Owner)
        );
        if !is_admin {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "Accès interdit".to_string(),
                }),
            ));
        }
    }

    let member = WorkspaceMemberRepository::find_by_ids(&auth_user.state.db, ws_id, user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;
    if member.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Membre non trouvé".to_string(),
            }),
        ));
    }

    if let Some(m) = &member {
        if matches!(m.role, Role::Owner) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Impossible de supprimer le propriétaire (owner)".to_string(),
                }),
            ));
        }
    }

    WorkspaceMemberRepository::delete(&auth_user.state.db, ws_id, user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    Ok(Json(RemoveWorkspaceMemberResponse {
        message: "Membre supprimé".to_string(),
    }))
}

pub async fn list_workspace_members_handler(
    auth_user: AuthUser,
    Path(workspace_id): Path<String>,
) -> Result<Json<ListWorkspaceMembersResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ws_id = Uuid::parse_str(&workspace_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "workspace_id invalide (UUID attendu)".to_string(),
            }),
        )
    })?;

    let existing = WorkspaceRepository::find_by_id(&auth_user.state.db, ws_id)
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

    if existing.owner_id != auth_user.user_id {
        let me = WorkspaceMemberRepository::find_by_ids(&auth_user.state.db, ws_id, auth_user.user_id)
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

    let members = WorkspaceMemberRepository::find_by_workspace(&auth_user.state.db, ws_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    let mut members_with_users = Vec::new();
    for member in members {
        let user = UserRepository::find_by_id(&auth_user.state.db, member.user_id)
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
                    error: format!("Utilisateur {} non trouvé", member.user_id),
                }),
            ))?;

        let role_str = match member.role {
            Role::Owner => "Owner".to_string(),
            Role::Admin => "Admin".to_string(),
            Role::Member => "Member".to_string(),
        };
        members_with_users.push(WorkspaceMemberWithUserResponse {
            workspace_id: member.workspace_id.to_string(),
            user_id: member.user_id.to_string(),
            user_email: user.email,
            user_name: user.name,
            role: role_str,
            joined_at: member.joined_at.map(|dt| dt.to_string()),
        });
    }

    let owner_in_list = members_with_users
        .iter()
        .any(|m| m.user_id == existing.owner_id.to_string());
    
    if !owner_in_list {
        let owner = UserRepository::find_by_id(&auth_user.state.db, existing.owner_id)
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
                    error: "Propriétaire du workspace non trouvé".to_string(),
                }),
            ))?;

        members_with_users.insert(0, WorkspaceMemberWithUserResponse {
            workspace_id: existing.id.to_string(),
            user_id: existing.owner_id.to_string(),
            user_email: owner.email,
            user_name: owner.name,
            role: "Owner".to_string(),
            joined_at: existing.created_at.map(|dt| dt.to_string()),
        });
    }

    Ok(Json(ListWorkspaceMembersResponse {
        members: members_with_users,
    }))
}
