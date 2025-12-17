//! Module frontend de l'application shrtnr.
//!
//! Ce module contient toute l'interface utilisateur construite avec Leptos :
//! - **Pages** : Toutes les pages de l'application (landing, dashboard, auth, etc.)
//! - **Composants** : Composants réutilisables (navigation, footer)
//! - **Design System** : Composants de base (Button, Card, Input, etc.)
//! - **Layouts** : Layouts pour les différentes sections (dashboard, etc.)
//! - **State Management** : Gestion de l'état avec Leptos signals
//!
//! ## Architecture
//!
//! Le frontend utilise Leptos avec :
//! - **SSR** : Rendu côté serveur pour un chargement initial rapide
//! - **Hydration** : Activation de la réactivité côté client après le chargement
//! - **Routing** : Système de routing intégré de Leptos
//!
//! ## Structure
//!
//! - [`app`] : Point d'entrée de l'application et configuration du router
//! - [`pages`] : Pages de l'application (landing, login, dashboard, etc.)
//! - [`components`] : Composants réutilisables au niveau application
//! - [`design_system`] : Système de design avec composants de base
//! - [`layouts`] : Layouts pour structurer les pages
//! - [`state`] : Gestion de l'état global (auth, workspaces)

pub mod app;
pub mod components;
pub mod design_system;
pub mod layouts;
pub mod pages;
pub mod state;

pub use app::{shell, App};
