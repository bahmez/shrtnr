use crate::backend::entities::shortened_link::{self, Entity as ShortenedLink};
use sea_orm::*;
use uuid::Uuid;

pub struct ShortenedLinkRepository;

impl ShortenedLinkRepository {
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

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<shortened_link::Model>, DbErr> {
        ShortenedLink::find_by_id(id).one(db).await
    }

    pub async fn find_by_short_code(
        db: &DatabaseConnection,
        short_code: &str,
    ) -> Result<Option<shortened_link::Model>, DbErr> {
        ShortenedLink::find()
            .filter(shortened_link::Column::ShortCode.eq(short_code))
            .one(db)
            .await
    }

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

    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, DbErr> {
        ShortenedLink::delete_by_id(id).exec(db).await
    }
}
