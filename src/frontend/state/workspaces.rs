use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;
#[cfg(feature = "hydrate")]
use std::future::Future;

#[cfg(feature = "hydrate")]
use gloo_net::http::Request;

use crate::frontend::state::AuthStore;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceSummary {
    pub id: String,
    pub name: String,
    pub owner_id: String,
}

#[derive(Clone)]
pub struct WorkspaceStore {
    #[cfg_attr(not(feature = "hydrate"), allow(dead_code))]
    auth_store: AuthStore,
    workspaces: RwSignal<Vec<WorkspaceSummary>>,
    selected_id: RwSignal<Option<String>>,
    loading: RwSignal<bool>,
    last_error: RwSignal<Option<String>>,
}

impl WorkspaceStore {
    pub fn new(auth_store: AuthStore) -> Self {
        Self {
            auth_store,
            workspaces: RwSignal::new(Vec::new()),
            selected_id: RwSignal::new(None),
            loading: RwSignal::new(false),
            last_error: RwSignal::new(None),
        }
    }

    pub fn workspaces(&self) -> ReadSignal<Vec<WorkspaceSummary>> {
        self.workspaces.read_only()
    }

    pub fn selected_workspace(&self) -> Memo<Option<WorkspaceSummary>> {
        let list = self.workspaces();
        let selected = self.selected_id;
        Memo::new(move |_| {
            let selected_id = selected.get();
            list.get()
                .into_iter()
                .find(|workspace| Some(&workspace.id) == selected_id.as_ref())
        })
    }

    pub fn loading(&self) -> ReadSignal<bool> {
        self.loading.read_only()
    }

    pub fn last_error(&self) -> ReadSignal<Option<String>> {
        self.last_error.read_only()
    }

    pub fn clear(&self) {
        self.workspaces.set(Vec::new());
        self.selected_id.set(None);
        self.loading.set(false);
        self.last_error.set(None);
    }

    pub fn select_workspace(&self, workspace_id: Option<String>) {
        if self.workspaces.get().is_empty() {
            self.selected_id.set(None);
            return;
        }

        if let Some(id) = workspace_id {
            if self.workspaces.get().iter().any(|ws| ws.id == id) {
                self.selected_id.set(Some(id));
            }
        } else {
            self.selected_id
                .set(self.workspaces.get().first().map(|ws| ws.id.clone()));
        }
    }

    #[cfg(feature = "hydrate")]
    pub fn load_for_owner(&self, owner_id: String) {
        if owner_id.is_empty() {
            self.clear();
            return;
        }

        let Some(token) = self.auth_store.access_token() else {
            self.clear();
            self.last_error.set(Some(
                "Connexion requise pour charger les workspaces.".into(),
            ));
            return;
        };

        self.loading.set(true);
        self.last_error.set(None);

        let workspaces = self.workspaces.clone();
        let selected_id = self.selected_id.clone();
        let loading = self.loading.clone();
        let last_error = self.last_error.clone();
        let token_clone = token.clone();
        spawn_local(async move {
            match fetch_workspaces(&owner_id, &token_clone).await {
                Ok(list) => {
                    loading.set(false);
                    workspaces.set(list.clone());
                    if selected_id.get().is_none()
                        || selected_id
                            .get()
                            .map(|current| !list.iter().any(|ws| ws.id == current))
                            .unwrap_or(false)
                    {
                        selected_id.set(list.first().map(|ws| ws.id.clone()));
                    }
                }
                Err(err) => {
                    loading.set(false);
                    last_error.set(Some(err));
                    workspaces.set(Vec::new());
                    selected_id.set(None);
                }
            }
        });
    }

    #[cfg(feature = "hydrate")]
    pub fn create_workspace(
        &self,
        owner_id: String,
        name: String,
    ) -> Option<impl Future<Output = Result<WorkspaceSummary, String>>> {
        let token = self.auth_store.access_token()?;
        let workspaces = self.workspaces.clone();
        let selected_id = self.selected_id.clone();

        Some(async move {
            let payload = CreateWorkspacePayload {
                name: name.clone(),
                owner_id: owner_id.clone(),
            };
            let created = create_workspace_request(&payload, &token).await?;
            let summary = WorkspaceSummary {
                id: created.id,
                name: payload.name,
                owner_id,
            };
            workspaces.update(|list| list.insert(0, summary.clone()));
            selected_id.set(Some(summary.id.clone()));
            Ok(summary)
        })
    }
}

pub fn provide_workspace_store(auth_store: AuthStore) -> WorkspaceStore {
    let store = WorkspaceStore::new(auth_store);
    provide_context(store.clone());
    store
}

pub fn use_workspace_store() -> WorkspaceStore {
    use_context::<WorkspaceStore>().expect("WorkspaceStore not provided")
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
struct ListWorkspacesResponse {
    workspaces: Vec<WorkspaceResponse>,
}

#[cfg(feature = "hydrate")]
#[derive(serde::Serialize)]
struct CreateWorkspacePayload {
    name: String,
    owner_id: String,
}

#[cfg(feature = "hydrate")]
async fn fetch_workspaces(owner_id: &str, token: &str) -> Result<Vec<WorkspaceSummary>, String> {
    let url = format!("/api/workspaces?owner_id={owner_id}");
    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        let parsed = response
            .json::<ListWorkspacesResponse>()
            .await
            .map_err(|err| format!("Réponse invalide: {err}"))?;
        Ok(parsed
            .workspaces
            .into_iter()
            .map(|ws| WorkspaceSummary {
                id: ws.id,
                name: ws.name,
                owner_id: ws.owner_id,
            })
            .collect())
    } else {
        let status = response.status();
        Err(format!(
            "Impossible de récupérer les workspaces (code {status})"
        ))
    }
}

#[cfg(feature = "hydrate")]
async fn create_workspace_request(
    payload: &CreateWorkspacePayload,
    token: &str,
) -> Result<WorkspaceResponse, String> {
    let response = Request::post("/api/workspaces")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {token}"))
        .json(payload)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.ok() {
        response
            .json::<WorkspaceResponse>()
            .await
            .map_err(|err| format!("Réponse invalide: {err}"))
    } else {
        let status = response.status();
        Err(format!("Impossible de créer le workspace (code {status})"))
    }
}
