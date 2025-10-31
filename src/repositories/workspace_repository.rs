use crate::entities::workspace::{self, Entity as Workspace};
use sea_orm::*;
use uuid::Uuid;

pub struct WorkspaceRepository;

impl WorkspaceRepository {
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

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<workspace::Model>, DbErr> {
        Workspace::find_by_id(id).one(db).await
    }

    pub async fn find_by_owner(
        db: &DatabaseConnection,
        owner_id: Uuid,
    ) -> Result<Vec<workspace::Model>, DbErr> {
        Workspace::find()
            .filter(workspace::Column::OwnerId.eq(owner_id))
            .all(db)
            .await
    }

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

    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, DbErr> {
        Workspace::delete_by_id(id).exec(db).await
    }

    pub async fn list(
        db: &DatabaseConnection,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<workspace::Model>, DbErr> {
        Workspace::find()
            .limit(limit)
            .offset(offset)
            .all(db)
            .await
    }
}

