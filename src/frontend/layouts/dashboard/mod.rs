//! Layout du dashboard de l'application.
//!
//! Ce module contient tous les composants nécessaires pour le layout du dashboard :
//! - [`DashboardLayout`] : Layout principal avec navbar et footer
//! - [`DashboardNavbar`] : Barre de navigation du dashboard
//! - [`DashboardFooter`] : Pied de page du dashboard
//! - [`WorkspaceModal`] : Modal pour créer un nouveau workspace
//! - [`SettingsModal`] : Modal pour modifier les paramètres du profil

mod footer;
mod layout;
mod navbar;
mod settings_modal;
mod workspace_modal;

pub use layout::DashboardLayout;
