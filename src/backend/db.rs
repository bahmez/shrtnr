//! Gestion de la connexion à la base de données.
//!
//! Ce module fournit les fonctions pour établir une connexion
//! à la base de données (SQLite ou PostgreSQL) et définit
//! le type alias `Db` pour simplifier l'utilisation.

use sea_orm::{Database, DatabaseConnection};

/// Établit une connexion à la base de données.
///
/// La fonction lit la variable d'environnement `DATABASE_URL` pour déterminer
/// quelle base de données utiliser. Si la variable n'est pas définie, elle
/// utilise SQLite par défaut avec le fichier `shrtnr.db`.
///
/// # Variables d'environnement
///
/// - `DATABASE_URL` : URL de connexion à la base de données
///   - SQLite : `sqlite:shrtnr.db?mode=rwc`
///   - PostgreSQL : `postgresql://user:password@localhost/dbname`
///
/// # Returns
///
/// * `Ok(DatabaseConnection)` - Connexion réussie
/// * `Err(DbErr)` - Erreur de connexion (base de données introuvable, credentials invalides, etc.)
///
/// # Exemple
///
/// ```rust,no_run
/// use shrtnr::backend::connect;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Utilise SQLite par défaut
/// let db = connect().await?;
///
/// // Ou avec PostgreSQL (via DATABASE_URL)
/// std::env::set_var("DATABASE_URL", "postgresql://user:pass@localhost/shrtnr");
/// let db = connect().await?;
/// # Ok(())
/// # }
/// ```
pub async fn connect() -> Result<DatabaseConnection, sea_orm::DbErr> {
    // Load .env if present (no-op if absent)
    let _ = dotenvy::dotenv();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:shrtnr.db?mode=rwc".to_string());

    Database::connect(database_url).await
}

/// Type alias pour `DatabaseConnection` de Sea-ORM.
///
/// Utilisé pour simplifier les signatures de fonctions et améliorer la lisibilité.
pub type Db = DatabaseConnection;
