mod page;
mod client;

pub use page::WorkspacePage;

#[cfg(feature = "hydrate")]
pub use client::{fetch_workspace_stats, fetch_recent_links};

#[derive(Clone, Debug)]
pub struct WorkspaceStats {
    pub workspace_id: String,
    pub total_links: u64,
    pub total_clicks: u64,
}

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

