//! Repositories pour l'accès aux données.
//!
//! Ce module implémente le pattern Repository pour abstraire l'accès
//! à la base de données. Chaque repository fournit des méthodes spécialisées
//! pour interagir avec une entité spécifique.
//!
//! ## Repositories disponibles
//!
//! - [`UserRepository`] : Gestion des utilisateurs
//! - [`WorkspaceRepository`] : Gestion des workspaces
//! - [`WorkspaceMemberRepository`] : Gestion des membres de workspace
//! - [`ShortenedLinkRepository`] : Gestion des liens raccourcis
//! - [`LinkClickRepository`] : Gestion des statistiques de clics

pub mod link_click_repository;
pub mod shortened_link_repository;
pub mod user_repository;
pub mod workspace_member_repository;
pub mod workspace_repository;

pub use link_click_repository::LinkClickRepository;
pub use shortened_link_repository::ShortenedLinkRepository;
pub use user_repository::UserRepository;
pub use workspace_member_repository::WorkspaceMemberRepository;
pub use workspace_repository::WorkspaceRepository;
