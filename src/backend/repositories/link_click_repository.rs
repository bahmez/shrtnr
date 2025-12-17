//! Repository pour la gestion des statistiques de clics.
//!
//! Fournit des méthodes pour enregistrer et interroger les clics sur les liens.

use crate::backend::entities::link_click::{self, Entity as LinkClick};
use sea_orm::*;
use uuid::Uuid;

/// Repository pour les opérations sur les clics de liens.
pub struct LinkClickRepository;

impl LinkClickRepository {
    /// Enregistre un nouveau clic sur un lien.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `link_id` - UUID du lien cliqué
    /// * `ip_address` - Adresse IP du client (optionnel)
    /// * `user_agent` - User-agent du navigateur (optionnel)
    /// * `referer` - URL de référence (optionnel)
    /// * `country` - Pays du client (optionnel, depuis géolocalisation)
    /// * `city` - Ville du client (optionnel, depuis géolocalisation)
    ///
    /// # Returns
    ///
    /// * `Ok(link_click::Model)` - Le clic enregistré avec son ID généré
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn create(
        db: &DatabaseConnection,
        link_id: Uuid,
        ip_address: Option<String>,
        user_agent: Option<String>,
        referer: Option<String>,
        country: Option<String>,
        city: Option<String>,
    ) -> Result<link_click::Model, DbErr> {
        let click = link_click::ActiveModel {
            id: Set(Uuid::new_v4()),
            link_id: Set(link_id),
            clicked_at: Set(Some(chrono::Utc::now().naive_utc())),
            ip_address: Set(ip_address),
            user_agent: Set(user_agent),
            referer: Set(referer),
            country: Set(country),
            city: Set(city),
        };

        click.insert(db).await
    }

    /// Liste les clics d'un lien avec pagination.
    ///
    /// Les clics sont triés par date décroissante (plus récents en premier).
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `link_id` - UUID du lien
    /// * `limit` - Nombre maximum de clics à retourner
    /// * `offset` - Nombre de clics à ignorer (pour la pagination)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<link_click::Model>)` - Liste des clics
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn find_by_link(
        db: &DatabaseConnection,
        link_id: Uuid,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<link_click::Model>, DbErr> {
        LinkClick::find()
            .filter(link_click::Column::LinkId.eq(link_id))
            .order_by_desc(link_click::Column::ClickedAt)
            .limit(limit)
            .offset(offset)
            .all(db)
            .await
    }

    /// Compte le nombre total de clics pour un lien.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `link_id` - UUID du lien
    ///
    /// # Returns
    ///
    /// * `Ok(u64)` - Nombre total de clics
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn count_by_link(db: &DatabaseConnection, link_id: Uuid) -> Result<u64, DbErr> {
        LinkClick::find()
            .filter(link_click::Column::LinkId.eq(link_id))
            .count(db)
            .await
    }

    /// Supprime tous les clics d'un lien.
    ///
    /// Utilisé lors de la suppression d'un lien pour nettoyer les statistiques.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `link_id` - UUID du lien
    ///
    /// # Returns
    ///
    /// * `Ok(DeleteResult)` - Résultat de la suppression
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn delete_by_link(
        db: &DatabaseConnection,
        link_id: Uuid,
    ) -> Result<DeleteResult, DbErr> {
        LinkClick::delete_many()
            .filter(link_click::Column::LinkId.eq(link_id))
            .exec(db)
            .await
    }
}
