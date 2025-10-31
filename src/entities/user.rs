use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    #[sea_orm(unique)]
    pub email: String,
    
    pub password_hash: String,
    
    pub name: Option<String>,
    
    pub created_at: Option<DateTime>,
    
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::workspace::Entity")]
    Workspaces,
    
    #[sea_orm(has_many = "super::workspace_member::Entity")]
    WorkspaceMembers,
    
    #[sea_orm(has_many = "super::shortened_link::Entity")]
    ShortenedLinks,
}

impl Related<super::workspace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workspaces.def()
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

