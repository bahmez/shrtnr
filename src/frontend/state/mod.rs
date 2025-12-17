//! Gestion de l'état global de l'application.
//!
//! Ce module utilise Leptos signals pour gérer l'état réactif partagé
//! entre les composants :
//! - [`AuthStore`] : État d'authentification (utilisateur connecté, tokens)
//! - [`WorkspaceStore`] : État des workspaces (workspace actif, liste des workspaces)
//!
//! Les stores sont fournis via le système de contexte de Leptos et peuvent
//! être accédés dans n'importe quel composant enfant.

pub mod auth;
pub mod workspaces;

pub use auth::{provide_auth_store, use_auth_store, AuthStore};
pub use workspaces::{
    provide_workspace_store, use_workspace_store, WorkspaceStore, WorkspaceSummary,
};
