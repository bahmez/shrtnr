//! Repository pour la gestion des liens raccourcis.
//!
//! Fournit des méthodes CRUD et des requêtes spécialisées
//! pour l'entité ShortenedLink dans la base de données.

use crate::backend::entities::shortened_link::{self, Entity as ShortenedLink};
use sea_orm::*;
use uuid::Uuid;

/// Repository pour les opérations sur les liens raccourcis.
pub struct ShortenedLinkRepository;

impl ShortenedLinkRepository {
    /// Crée un nouveau lien raccourci dans la base de données.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `short_code` - Code court unique du lien
    /// * `original_url` - URL originale à raccourcir
    /// * `workspace_id` - ID du workspace auquel appartient le lien
    /// * `user_id` - ID de l'utilisateur créateur
    /// * `title` - Titre optionnel du lien
    /// * `expires_at` - Date d'expiration optionnelle
    ///
    /// # Returns
    ///
    /// * `Ok(shortened_link::Model)` - Le lien créé avec son ID généré
    /// * `Err(DbErr)` - En cas d'erreur (code court déjà existant, etc.)
    pub async fn create(
        db: &DatabaseConnection,
        short_code: String,
        original_url: String,
        workspace_id: Uuid,
        user_id: Uuid,
        title: Option<String>,
        expires_at: Option<chrono::NaiveDateTime>,
    ) -> Result<shortened_link::Model, DbErr> {
        let link = shortened_link::ActiveModel {
            id: Set(Uuid::new_v4()),
            short_code: Set(short_code),
            original_url: Set(original_url),
            workspace_id: Set(workspace_id),
            user_id: Set(user_id),
            title: Set(title),
            created_at: Set(Some(chrono::Utc::now().naive_utc())),
            expires_at: Set(expires_at),
            is_active: Set(true),
        };

        link.insert(db).await
    }

    /// Trouve un lien par son identifiant unique.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `id` - UUID du lien
    ///
    /// # Returns
    ///
    /// * `Ok(Some(shortened_link::Model))` - Le lien trouvé
    /// * `Ok(None)` - Si aucun lien n'est trouvé
    /// * `Err(DbErr)` - En cas d'erreur de base de données
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<shortened_link::Model>, DbErr> {
        ShortenedLink::find_by_id(id).one(db).await
    }

    /// Trouve un lien par son code court.
    ///
    /// Utilisé lors de la redirection pour trouver l'URL originale.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `short_code` - Code court du lien (ex: "abc123")
    ///
    /// # Returns
    ///
    /// * `Ok(Some(shortened_link::Model))` - Le lien trouvé
    /// * `Ok(None)` - Si aucun lien n'est trouvé
    /// * `Err(DbErr)` - En cas d'erreur de base de données
    pub async fn find_by_short_code(
        db: &DatabaseConnection,
        short_code: &str,
    ) -> Result<Option<shortened_link::Model>, DbErr> {
        ShortenedLink::find()
            .filter(shortened_link::Column::ShortCode.eq(short_code))
            .one(db)
            .await
    }

    /// Liste les liens d'un workspace avec pagination.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `workspace_id` - UUID du workspace
    /// * `limit` - Nombre maximum de liens à retourner
    /// * `offset` - Nombre de liens à ignorer (pour la pagination)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<shortened_link::Model>)` - Liste des liens
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn find_by_workspace(
        db: &DatabaseConnection,
        workspace_id: Uuid,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<shortened_link::Model>, DbErr> {
        ShortenedLink::find()
            .filter(shortened_link::Column::WorkspaceId.eq(workspace_id))
            .limit(limit)
            .offset(offset)
            .all(db)
            .await
    }

    /// Liste les liens créés par un utilisateur avec pagination.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `user_id` - UUID de l'utilisateur
    /// * `limit` - Nombre maximum de liens à retourner
    /// * `offset` - Nombre de liens à ignorer (pour la pagination)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<shortened_link::Model>)` - Liste des liens
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn find_by_user(
        db: &DatabaseConnection,
        user_id: Uuid,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<shortened_link::Model>, DbErr> {
        ShortenedLink::find()
            .filter(shortened_link::Column::UserId.eq(user_id))
            .limit(limit)
            .offset(offset)
            .all(db)
            .await
    }

    /// Met à jour un lien raccourci.
    ///
    /// Seuls les champs fournis (non-None) seront mis à jour.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `id` - UUID du lien à mettre à jour
    /// * `title` - Nouveau titre (optionnel)
    /// * `original_url` - Nouvelle URL originale (optionnel)
    /// * `is_active` - Nouvel état actif/inactif (optionnel)
    ///
    /// # Returns
    ///
    /// * `Ok(shortened_link::Model)` - Le lien mis à jour
    /// * `Err(DbErr)` - Si le lien n'existe pas ou en cas d'erreur
    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        title: Option<String>,
        original_url: Option<String>,
        is_active: Option<bool>,
    ) -> Result<shortened_link::Model, DbErr> {
        let link = ShortenedLink::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Link not found".to_string()))?;

        let mut link: shortened_link::ActiveModel = link.into();

        if let Some(t) = title {
            link.title = Set(Some(t));
        }
        if let Some(url) = original_url {
            link.original_url = Set(url);
        }
        if let Some(active) = is_active {
            link.is_active = Set(active);
        }

        link.update(db).await
    }

    /// Supprime un lien raccourci de la base de données.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `id` - UUID du lien à supprimer
    ///
    /// # Returns
    ///
    /// * `Ok(DeleteResult)` - Résultat de la suppression
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, DbErr> {
        ShortenedLink::delete_by_id(id).exec(db).await
    }
}
