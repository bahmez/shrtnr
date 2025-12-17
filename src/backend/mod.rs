//! Module backend de l'application shrtnr.
//!
//! Ce module contient toute la logique backend de l'application :
//! - **API REST** : Handlers Axum pour tous les endpoints
//! - **Authentification** : JWT, hashage de mots de passe, gestion des sessions
//! - **Gestion de données** : Entities Sea-ORM et repositories
//! - **Analytics** : Tracking des clics et statistiques
//! - **Workspaces** : Gestion des espaces de travail collaboratifs
//!
//! ## Architecture
//!
//! Le backend suit une architecture en couches :
//! - **Handlers** : Points d'entrée HTTP (dans les modules `auth`, `links`, `workspaces`, etc.)
//! - **Repositories** : Abstraction de l'accès aux données
//! - **Entities** : Modèles de données Sea-ORM
//! - **Middleware** : Authentification, CORS, gestion d'erreurs
//!
//! ## Exemple
//!
//! ```rust,no_run
//! use shrtnr::backend::{connect, AppState};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let db = connect().await?;
//! let app_state = AppState::new(db, "jwt-secret");
//! # Ok(())
//! # }
//! ```

pub mod analytics;
pub mod auth;
pub mod config;
pub mod db;
pub mod entities;
pub mod links;
pub mod middleware;
pub mod repositories;
pub mod stats;
pub mod workspaces;

pub use config::AppState;
pub use db::{connect, Db};
