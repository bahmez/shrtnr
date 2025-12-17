//! Repository pour la gestion des membres de workspace.
//!
//! Fournit des méthodes CRUD pour l'entité WorkspaceMember dans la base de données.

use crate::backend::entities::workspace_member::{self, Entity as WorkspaceMember, Role};
use sea_orm::*;
use uuid::Uuid;

/// Repository pour les opérations sur les membres de workspace.
pub struct WorkspaceMemberRepository;

impl WorkspaceMemberRepository {
    /// Ajoute un membre à un workspace.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `workspace_id` - UUID du workspace
    /// * `user_id` - UUID de l'utilisateur à ajouter
    /// * `role` - Rôle du membre (owner, admin, member)
    ///
    /// # Returns
    ///
    /// * `Ok(workspace_member::Model)` - Le membre créé
    /// * `Err(DbErr)` - En cas d'erreur (membre déjà existant, etc.)
    pub async fn create(
        db: &DatabaseConnection,
        workspace_id: Uuid,
        user_id: Uuid,
        role: Role,
    ) -> Result<workspace_member::Model, DbErr> {
        let member = workspace_member::ActiveModel {
            workspace_id: Set(workspace_id),
            user_id: Set(user_id),
            role: Set(role),
            joined_at: Set(Some(chrono::Utc::now().naive_utc())),
        };

        member.insert(db).await
    }

    /// Trouve un membre spécifique dans un workspace.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `workspace_id` - UUID du workspace
    /// * `user_id` - UUID de l'utilisateur
    ///
    /// # Returns
    ///
    /// * `Ok(Some(workspace_member::Model))` - Le membre trouvé
    /// * `Ok(None)` - Si l'utilisateur n'est pas membre du workspace
    /// * `Err(DbErr)` - En cas d'erreur de base de données
    pub async fn find_by_ids(
        db: &DatabaseConnection,
        workspace_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<workspace_member::Model>, DbErr> {
        WorkspaceMember::find()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_member::Column::UserId.eq(user_id))
            .one(db)
            .await
    }

    /// Liste tous les membres d'un workspace.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `workspace_id` - UUID du workspace
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<workspace_member::Model>)` - Liste des membres
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn find_by_workspace(
        db: &DatabaseConnection,
        workspace_id: Uuid,
    ) -> Result<Vec<workspace_member::Model>, DbErr> {
        WorkspaceMember::find()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .all(db)
            .await
    }

    /// Liste tous les workspaces dont un utilisateur est membre.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `user_id` - UUID de l'utilisateur
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<workspace_member::Model>)` - Liste des membreships
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn find_by_user(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<workspace_member::Model>, DbErr> {
        WorkspaceMember::find()
            .filter(workspace_member::Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    /// Met à jour le rôle d'un membre dans un workspace.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `workspace_id` - UUID du workspace
    /// * `user_id` - UUID de l'utilisateur
    /// * `role` - Nouveau rôle (owner, admin, member)
    ///
    /// # Returns
    ///
    /// * `Ok(workspace_member::Model)` - Le membre mis à jour
    /// * `Err(DbErr)` - Si le membre n'existe pas ou en cas d'erreur
    pub async fn update_role(
        db: &DatabaseConnection,
        workspace_id: Uuid,
        user_id: Uuid,
        role: Role,
    ) -> Result<workspace_member::Model, DbErr> {
        let member =
            Self::find_by_ids(db, workspace_id, user_id)
                .await?
                .ok_or(DbErr::RecordNotFound(
                    "Workspace member not found".to_string(),
                ))?;

        let mut member: workspace_member::ActiveModel = member.into();
        member.role = Set(role);

        member.update(db).await
    }

    /// Retire un membre d'un workspace.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `workspace_id` - UUID du workspace
    /// * `user_id` - UUID de l'utilisateur à retirer
    ///
    /// # Returns
    ///
    /// * `Ok(DeleteResult)` - Résultat de la suppression
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn delete(
        db: &DatabaseConnection,
        workspace_id: Uuid,
        user_id: Uuid,
    ) -> Result<DeleteResult, DbErr> {
        WorkspaceMember::delete_many()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_member::Column::UserId.eq(user_id))
            .exec(db)
            .await
    }
}
