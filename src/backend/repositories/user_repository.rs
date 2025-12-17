//! Repository pour la gestion des utilisateurs.
//!
//! Fournit des méthodes CRUD et des requêtes spécialisées
//! pour l'entité User dans la base de données.

use crate::backend::entities::user::{self, Entity as User};
use sea_orm::*;
use uuid::Uuid;

/// Repository pour les opérations sur les utilisateurs.
pub struct UserRepository;

impl UserRepository {
    /// Crée un nouvel utilisateur dans la base de données.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `email` - Adresse email de l'utilisateur (doit être unique)
    /// * `password_hash` - Hash bcrypt du mot de passe
    /// * `name` - Nom complet de l'utilisateur (optionnel)
    ///
    /// # Returns
    ///
    /// * `Ok(user::Model)` - L'utilisateur créé avec son ID généré
    /// * `Err(DbErr)` - En cas d'erreur (email déjà existant, etc.)
    pub async fn create(
        db: &DatabaseConnection,
        email: String,
        password_hash: String,
        name: Option<String>,
    ) -> Result<user::Model, DbErr> {
        let user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(email),
            password_hash: Set(password_hash),
            name: Set(name),
            created_at: Set(Some(chrono::Utc::now().naive_utc())),
            updated_at: Set(Some(chrono::Utc::now().naive_utc())),
        };

        user.insert(db).await
    }

    /// Trouve un utilisateur par son identifiant unique.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `id` - UUID de l'utilisateur
    ///
    /// # Returns
    ///
    /// * `Ok(Some(user::Model))` - L'utilisateur trouvé
    /// * `Ok(None)` - Si aucun utilisateur n'est trouvé
    /// * `Err(DbErr)` - En cas d'erreur de base de données
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    /// Trouve un utilisateur par son adresse email.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `email` - Adresse email à rechercher
    ///
    /// # Returns
    ///
    /// * `Ok(Some(user::Model))` - L'utilisateur trouvé
    /// * `Ok(None)` - Si aucun utilisateur n'est trouvé
    /// * `Err(DbErr)` - En cas d'erreur de base de données
    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    /// Met à jour les informations d'un utilisateur.
    ///
    /// Seuls les champs fournis (non-None) seront mis à jour.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `id` - UUID de l'utilisateur à mettre à jour
    /// * `name` - Nouveau nom (optionnel, laisse inchangé si None)
    /// * `email` - Nouvelle adresse email (optionnel, doit être unique si fourni)
    ///
    /// # Returns
    ///
    /// * `Ok(user::Model)` - L'utilisateur mis à jour
    /// * `Err(DbErr)` - Si l'utilisateur n'existe pas ou en cas d'erreur
    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        name: Option<String>,
        email: Option<String>,
    ) -> Result<user::Model, DbErr> {
        let user = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;

        let mut user: user::ActiveModel = user.into();

        if let Some(n) = name {
            user.name = Set(Some(n));
        }
        if let Some(e) = email {
            user.email = Set(e);
        }
        user.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

        user.update(db).await
    }

    /// Supprime un utilisateur de la base de données.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `id` - UUID de l'utilisateur à supprimer
    ///
    /// # Returns
    ///
    /// * `Ok(DeleteResult)` - Résultat de la suppression
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, DbErr> {
        User::delete_by_id(id).exec(db).await
    }

    /// Liste les utilisateurs avec pagination.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `limit` - Nombre maximum d'utilisateurs à retourner
    /// * `offset` - Nombre d'utilisateurs à ignorer (pour la pagination)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<user::Model>)` - Liste des utilisateurs
    /// * `Err(DbErr)` - En cas d'erreur
    pub async fn list(
        db: &DatabaseConnection,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<user::Model>, DbErr> {
        User::find().limit(limit).offset(offset).all(db).await
    }
}
