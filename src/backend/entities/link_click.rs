//! Entity LinkClick pour la table `link_clicks`.
//!
//! Représente un clic sur un lien raccourci avec toutes les métadonnées
//! collectées (IP, user-agent, referer, géolocalisation).

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Modèle de données pour un clic sur un lien.
///
/// Enregistre chaque clic sur un lien raccourci avec :
/// - Timestamp du clic
/// - Informations du client (IP, user-agent, referer)
/// - Géolocalisation (pays, ville) si disponible
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "link_clicks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub link_id: Uuid,

    pub clicked_at: Option<DateTime>,

    pub ip_address: Option<String>,

    pub user_agent: Option<String>,

    pub referer: Option<String>,

    pub country: Option<String>,

    pub city: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::shortened_link::Entity",
        from = "Column::LinkId",
        to = "super::shortened_link::Column::Id"
    )]
    ShortenedLink,
}

impl Related<super::shortened_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShortenedLink.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
