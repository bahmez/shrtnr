//! Entity Workspace pour la table `workspaces`.
//!
//! Représente un espace de travail collaboratif où les utilisateurs
//! peuvent créer et gérer des liens raccourcis ensemble.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Modèle de données pour un workspace.
///
/// Un workspace est un espace de travail collaboratif qui permet
/// à plusieurs utilisateurs de gérer des liens raccourcis ensemble.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workspaces")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub name: String,

    pub owner_id: Uuid,

    pub created_at: Option<DateTime>,

    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id"
    )]
    Owner,

    #[sea_orm(has_many = "super::workspace_member::Entity")]
    WorkspaceMembers,

    #[sea_orm(has_many = "super::shortened_link::Entity")]
    ShortenedLinks,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl Related<super::workspace_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkspaceMembers.def()
    }
}

impl Related<super::shortened_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShortenedLinks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
