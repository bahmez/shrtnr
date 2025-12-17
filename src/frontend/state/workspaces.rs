//! Gestion de l'état des workspaces.
//!
//! Ce module fournit un store réactif pour gérer les workspaces de l'utilisateur,
//! incluant la liste des workspaces et le workspace actuellement sélectionné.

use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos::task::spawn_local;
#[cfg(feature = "hydrate")]
use std::future::Future;

#[cfg(feature = "hydrate")]
use gloo_net::http::Request;

use crate::frontend::state::AuthStore;

/// Résumé d'un workspace pour l'affichage dans l'interface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceSummary {
    /// Identifiant unique du workspace
    pub id: String,
    /// Nom du workspace
    pub name: String,
    /// ID du propriétaire du workspace
    pub owner_id: String,
}

/// Store réactif pour l'état des workspaces.
///
/// Utilise Leptos signals pour gérer l'état de manière réactive.
/// Le store maintient la liste des workspaces accessibles et le workspace sélectionné.
#[derive(Clone, Copy)]
pub struct WorkspaceStore {
    /// Référence au store d'authentification (pour obtenir le token)
    #[cfg_attr(not(feature = "hydrate"), allow(dead_code))]
    auth_store: AuthStore,
    /// Liste des workspaces accessibles
    workspaces: RwSignal<Vec<WorkspaceSummary>>,
    /// ID du workspace actuellement sélectionné
    selected_id: RwSignal<Option<String>>,
    /// Indique si un chargement est en cours
    loading: RwSignal<bool>,
    /// Dernière erreur (optionnel)
    last_error: RwSignal<Option<String>>,
}

impl WorkspaceStore {
    /// Crée une nouvelle instance de `WorkspaceStore`.
    ///
    /// # Arguments
    ///
    /// * `auth_store` - Le store d'authentification pour accéder aux tokens
    pub fn new(auth_store: AuthStore) -> Self {
        Self {
            auth_store,
            workspaces: RwSignal::new(Vec::new()),
            selected_id: RwSignal::new(None),
            loading: RwSignal::new(false),
            last_error: RwSignal::new(None),
        }
    }

    /// Retourne un signal en lecture seule pour la liste des workspaces.
    ///
    /// # Returns
    ///
    /// Signal contenant la liste des workspaces accessibles.
    pub fn workspaces(&self) -> ReadSignal<Vec<WorkspaceSummary>> {
        self.workspaces.read_only()
    }

    /// Retourne un signal en lecture seule pour l'ID du workspace sélectionné.
    ///
    /// # Returns
    ///
    /// Signal contenant `Some(String)` si un workspace est sélectionné, `None` sinon.
    pub fn selected_id(&self) -> ReadSignal<Option<String>> {
        self.selected_id.read_only()
    }

    /// Retourne un signal pour le workspace actuellement sélectionné.
    ///
    /// Le signal est dérivé de `workspaces` et `selected_id` pour retourner
    /// automatiquement le workspace correspondant à l'ID sélectionné.
    ///
    /// # Returns
    ///
    /// Signal contenant `Some(WorkspaceSummary)` si un workspace est sélectionné, `None` sinon.
    pub fn selected_workspace(&self) -> Signal<Option<WorkspaceSummary>> {
        let workspaces = self.workspaces();
        let selected_id = self.selected_id();
        Signal::derive(move || {
            let selected_id_value = selected_id.get();
            if let Some(id) = selected_id_value.as_ref() {
                workspaces.get()
                    .into_iter()
                    .find(|workspace| &workspace.id == id)
            } else {
                None
            }
        })
    }

    /// Retourne un signal indiquant si un chargement est en cours.
    pub fn loading(&self) -> ReadSignal<bool> {
        self.loading.read_only()
    }

    /// Retourne un signal pour la dernière erreur.
    ///
    /// # Returns
    ///
    /// Signal contenant `Some(String)` en cas d'erreur, `None` sinon.
    pub fn last_error(&self) -> ReadSignal<Option<String>> {
        self.last_error.read_only()
    }

    /// Réinitialise le store (vide la liste, désélectionne le workspace).
    pub fn clear(&self) {
        self.workspaces.set(Vec::new());
        self.selected_id.set(None);
        self.loading.set(false);
        self.last_error.set(None);
    }

    /// Sélectionne un workspace par son ID.
    ///
    /// Si `workspace_id` est `None`, sélectionne le premier workspace de la liste.
    /// Si la liste est vide ou si l'ID fourni n'existe pas, désélectionne le workspace.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - ID du workspace à sélectionner (ou `None` pour sélectionner le premier)
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

    /// Charge les workspaces d'un propriétaire depuis l'API.
    ///
    /// Charge de manière asynchrone tous les workspaces dont l'utilisateur est propriétaire
    /// ou membre, puis met à jour le store.
    ///
    /// # Arguments
    ///
    /// * `owner_id` - ID de l'utilisateur propriétaire
    ///
    /// # Note
    ///
    /// Cette fonction est uniquement disponible côté client (feature "hydrate").
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

    /// Crée un nouveau workspace via l'API.
    ///
    /// Crée le workspace, l'ajoute à la liste, et le sélectionne automatiquement.
    ///
    /// # Arguments
    ///
    /// * `owner_id` - ID du propriétaire du workspace
    /// * `name` - Nom du workspace à créer
    ///
    /// # Returns
    ///
    /// * `Some(Future)` - Future qui retourne le workspace créé ou une erreur
    /// * `None` - Si aucun token d'accès n'est disponible
    ///
    /// # Note
    ///
    /// Cette fonction est uniquement disponible côté client (feature "hydrate").
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

    /// Met à jour un workspace dans le store après modification.
    ///
    /// Met à jour le nom du workspace dans la liste si le workspace existe.
    ///
    /// # Arguments
    ///
    /// * `updated` - Workspace avec les informations mises à jour
    pub fn update_workspace_in_store(&self, updated: WorkspaceSummary) {
        self.workspaces.update(|workspaces| {
            if let Some(ws) = workspaces.iter_mut().find(|w| w.id == updated.id) {
                ws.name = updated.name.clone();
            }
        });
    }

}


/// Fournit le store des workspaces dans le contexte Leptos.
///
/// Crée un nouveau `WorkspaceStore` et l'ajoute au contexte pour qu'il soit accessible
/// dans tous les composants enfants.
///
/// # Arguments
///
/// * `auth_store` - Le store d'authentification (nécessaire pour accéder aux tokens)
///
/// # Returns
///
/// L'instance du `WorkspaceStore` créée.
///
/// # Note
///
/// Cette fonction doit être appelée une seule fois au niveau racine de l'application,
/// après `provide_auth_store()`.
/// Fournit le store des workspaces dans le contexte Leptos.
///
/// Crée un nouveau `WorkspaceStore` et l'ajoute au contexte pour qu'il soit accessible
/// dans tous les composants enfants.
///
/// # Arguments
///
/// * `auth_store` - Le store d'authentification (nécessaire pour accéder aux tokens)
///
/// # Returns
///
/// L'instance du `WorkspaceStore` créée.
///
/// # Note
///
/// Cette fonction doit être appelée une seule fois au niveau racine de l'application,
/// après `provide_auth_store()`.
pub fn provide_workspace_store(auth_store: AuthStore) -> WorkspaceStore {
    let store = WorkspaceStore::new(auth_store);
    provide_context(store.clone());
    store
}

/// Récupère le store des workspaces depuis le contexte Leptos.
///
/// # Returns
///
/// L'instance du `WorkspaceStore` depuis le contexte.
///
/// # Panics
///
/// Panique si `WorkspaceStore` n'a pas été fourni dans le contexte (si `provide_workspace_store()`
/// n'a pas été appelé dans un composant parent).
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

