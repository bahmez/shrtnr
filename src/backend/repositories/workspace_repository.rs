//! Repository pour la gestion des workspaces.
//!
//! Fournit des méthodes CRUD pour l'entité Workspace dans la base de données.

use crate::backend::entities::workspace::{self, Entity as Workspace};
use sea_orm::*;
use uuid::Uuid;

/// Repository pour les opérations sur les workspaces.
pub struct WorkspaceRepository;

impl WorkspaceRepository {
    /// Crée un nouveau workspace dans la base de données.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `name` - Nom du workspace
    /// * `owner_id` - UUID du propriétaire du workspace
    ///
    /// # Returns
    ///
    /// * `Ok(workspace::Model)` - Le workspace créé avec son ID généré
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        owner_id: Uuid,
    ) -> Result<workspace::Model, DbErr> {
        let workspace = workspace::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name),
            owner_id: Set(owner_id),
            created_at: Set(Some(chrono::Utc::now().naive_utc())),
            updated_at: Set(Some(chrono::Utc::now().naive_utc())),
        };

        workspace.insert(db).await
    }

    /// Trouve un workspace par son identifiant unique.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `id` - UUID du workspace
    ///
    /// # Returns
    ///
    /// * `Ok(Some(workspace::Model))` - Le workspace trouvé
    /// * `Ok(None)` - Si aucun workspace n'est trouvé
    /// * `Err(DbErr)` - En cas d'erreur de base de données
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<workspace::Model>, DbErr> {
        Workspace::find_by_id(id).one(db).await
    }

    /// Trouve tous les workspaces dont un utilisateur est propriétaire.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `owner_id` - UUID du propriétaire
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<workspace::Model>)` - Liste des workspaces
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn find_by_owner(
        db: &DatabaseConnection,
        owner_id: Uuid,
    ) -> Result<Vec<workspace::Model>, DbErr> {
        Workspace::find()
            .filter(workspace::Column::OwnerId.eq(owner_id))
            .all(db)
            .await
    }

    /// Met à jour un workspace.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `id` - UUID du workspace à mettre à jour
    /// * `name` - Nouveau nom (optionnel, laisse inchangé si None)
    ///
    /// # Returns
    ///
    /// * `Ok(workspace::Model)` - Le workspace mis à jour
    /// * `Err(DbErr)` - Si le workspace n'existe pas ou en cas d'erreur
    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        name: Option<String>,
    ) -> Result<workspace::Model, DbErr> {
        let workspace = Workspace::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Workspace not found".to_string()))?;

        let mut workspace: workspace::ActiveModel = workspace.into();

        if let Some(n) = name {
            workspace.name = Set(n);
        }
        workspace.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

        workspace.update(db).await
    }

    /// Supprime un workspace de la base de données.
    ///
    /// La suppression en cascade supprime également tous les membres et liens associés.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `id` - UUID du workspace à supprimer
    ///
    /// # Returns
    ///
    /// * `Ok(DeleteResult)` - Résultat de la suppression
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, DbErr> {
        Workspace::delete_by_id(id).exec(db).await
    }

    /// Liste tous les workspaces avec pagination.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `limit` - Nombre maximum de workspaces à retourner
    /// * `offset` - Nombre de workspaces à ignorer (pour la pagination)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<workspace::Model>)` - Liste des workspaces
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn list(
        db: &DatabaseConnection,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<workspace::Model>, DbErr> {
        Workspace::find().limit(limit).offset(offset).all(db).await
    }
}
