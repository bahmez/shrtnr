mod page;
#[cfg(feature = "hydrate")]
mod client;

pub use page::LinksPage;

#[cfg(feature = "hydrate")]
pub use client::{
    create_link, delete_link, fetch_links, update_link, LinkDetail, LinkListData,
};

#[derive(Clone, Debug)]
pub struct LinkItem {
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
