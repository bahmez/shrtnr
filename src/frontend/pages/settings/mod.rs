mod page;
#[cfg(feature = "hydrate")]
mod client;

pub use page::SettingsPage;
#[cfg(feature = "hydrate")]
pub use client::update_profile;

