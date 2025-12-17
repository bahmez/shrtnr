//! Entities Sea-ORM pour la base de données.
//!
//! Ce module contient toutes les entités de données correspondant aux tables
//! de la base de données. Chaque entité est générée par Sea-ORM et fournit
//! les méthodes CRUD de base.
//!
//! ## Entities disponibles
//!
//! - [`User`] : Utilisateurs de l'application
//! - [`Workspace`] : Espaces de travail collaboratifs
//! - [`WorkspaceMember`] : Membres et rôles dans les workspaces
//! - [`ShortenedLink`] : Liens raccourcis
//! - [`LinkClick`] : Statistiques de clics sur les liens

pub mod link_click;
pub mod shortened_link;
pub mod user;
pub mod workspace;
pub mod workspace_member;

pub use link_click::Entity as LinkClick;
pub use shortened_link::Entity as ShortenedLink;
pub use user::Entity as User;
pub use workspace::Entity as Workspace;
pub use workspace_member::Entity as WorkspaceMember;
