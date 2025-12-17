//! Module de gestion des workspaces.
//!
//! Fournit la page de workspace et les fonctions client pour récupérer
//! les statistiques et liens récents d'un workspace.

mod page;
mod client;

pub use page::WorkspacePage;

#[cfg(feature = "hydrate")]
pub use client::{fetch_workspace_stats, fetch_recent_links};

/// Statistiques d'un workspace.
#[derive(Clone, Debug)]
pub struct WorkspaceStats {
    pub workspace_id: String,
    pub total_links: u64,
    pub total_clicks: u64,
}

/// Résumé d'un lien pour l'affichage dans le workspace.
#[derive(Clone, Debug)]
pub struct LinkSummary {
    pub id: String,
    pub short_code: String,
    pub original_url: String,
    pub title: Option<String>,
    pub short_url: String,
    pub created_at: Option<String>,
    pub is_active: bool,
}

