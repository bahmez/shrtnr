//! Pages de l'application frontend.
//!
//! Ce module contient toutes les pages de l'application organisées par fonctionnalité :
//! - [`landing`] : Page d'accueil marketing
//! - [`auth`] : Pages d'authentification (login, register)
//! - [`workspace`] : Gestion des workspaces
//! - [`links`] : Gestion des liens raccourcis
//! - [`analytics`] : Analytics et visualisations
//! - [`settings`] : Paramètres utilisateur
//! - Pages d'information : [`support`], [`status`], [`aide`]

pub mod aide;
pub mod analytics;
pub mod auth;
pub mod landing;
pub mod layout_test;
pub mod links;
pub mod settings;
pub mod status;
pub mod support;
pub mod workspace;

pub use aide::AidePage;
pub use analytics::AnalyticsPage;
pub use auth::{LoginPage, RegisterPage};
pub use landing::LandingPage;
pub use layout_test::LayoutTestPage;
pub use links::LinksPage;
pub use settings::SettingsPage;
pub use status::StatusPage;
pub use support::SupportPage;
pub use workspace::WorkspacePage;
