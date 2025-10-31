use crate::entities::link_click::{self, Entity as LinkClick};
use sea_orm::*;
use uuid::Uuid;

pub struct LinkClickRepository;

impl LinkClickRepository {
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

    pub async fn count_by_link(
        db: &DatabaseConnection,
        link_id: Uuid,
    ) -> Result<u64, DbErr> {
        LinkClick::find()
            .filter(link_click::Column::LinkId.eq(link_id))
            .count(db)
            .await
    }

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

