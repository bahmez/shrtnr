use crate::entities::workspace_member::{self, Entity as WorkspaceMember, Role};
use sea_orm::*;
use uuid::Uuid;

pub struct WorkspaceMemberRepository;

impl WorkspaceMemberRepository {
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

    pub async fn find_by_workspace(
        db: &DatabaseConnection,
        workspace_id: Uuid,
    ) -> Result<Vec<workspace_member::Model>, DbErr> {
        WorkspaceMember::find()
            .filter(workspace_member::Column::WorkspaceId.eq(workspace_id))
            .all(db)
            .await
    }

    pub async fn find_by_user(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<workspace_member::Model>, DbErr> {
        WorkspaceMember::find()
            .filter(workspace_member::Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn update_role(
        db: &DatabaseConnection,
        workspace_id: Uuid,
        user_id: Uuid,
        role: Role,
    ) -> Result<workspace_member::Model, DbErr> {
        let member = Self::find_by_ids(db, workspace_id, user_id)
            .await?
            .ok_or(DbErr::RecordNotFound("Workspace member not found".to_string()))?;

        let mut member: workspace_member::ActiveModel = member.into();
        member.role = Set(role);

        member.update(db).await
    }

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

