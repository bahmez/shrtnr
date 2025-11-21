use crate::backend::entities::user::{self, Entity as User};
use sea_orm::*;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
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

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

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

    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, DbErr> {
        User::delete_by_id(id).exec(db).await
    }

    pub async fn list(
        db: &DatabaseConnection,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<user::Model>, DbErr> {
        User::find().limit(limit).offset(offset).all(db).await
    }
}
