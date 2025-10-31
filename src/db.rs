use sea_orm::{Database, DatabaseConnection};

pub async fn connect() -> Result<DatabaseConnection, sea_orm::DbErr> {
    // Load .env if present (no-op if absent)
    let _ = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:shrtnr.db?mode=rwc".to_string());

    Database::connect(database_url).await
}

pub type Db = DatabaseConnection;


