# Exemples de Documentation de Code

Ce fichier montre des exemples concrets de documentation pour le projet shrtnr.

## Exemple 1 : Handler API

```rust
/// Inscrit un nouvel utilisateur dans le système.
///
/// Crée un compte utilisateur avec un email et un mot de passe hashé,
/// puis génère une paire de tokens JWT (access et refresh) pour l'authentification.
///
/// # Arguments
///
/// * `state` - L'état de l'application contenant la connexion DB et la config JWT
/// * `payload` - Les données d'inscription (email, password, name optionnel)
///
/// # Returns
///
/// Retourne une `AuthResponse` contenant :
/// - Les informations de l'utilisateur créé
/// - Un access token JWT
/// - Un refresh token JWT
///
/// # Errors
///
/// * `409 Conflict` - Si l'email est déjà utilisé
/// * `500 Internal Server Error` - En cas d'erreur de hashage ou de création en DB
///
/// # Exemple
///
/// ```json
/// POST /api/auth/register
/// {
///   "email": "user@example.com",
///   "password": "securepassword123",
///   "name": "John Doe"
/// }
/// ```
pub async fn register_handler(
    Extension(state): Extension<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    // ...
}
```

## Exemple 2 : Struct de Requête

```rust
/// Requête pour l'inscription d'un nouvel utilisateur.
///
/// Tous les champs sont requis sauf `name` qui est optionnel.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Adresse email de l'utilisateur (doit être unique)
    pub email: String,
    /// Mot de passe en clair (sera hashé avant stockage)
    pub password: String,
    /// Nom complet de l'utilisateur (optionnel)
    pub name: Option<String>,
}
```

## Exemple 3 : Repository Method

```rust
impl UserRepository {
    /// Trouve un utilisateur par son adresse email.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `email` - L'adresse email à rechercher
    ///
    /// # Returns
    ///
    /// * `Ok(Some(User))` - Si l'utilisateur est trouvé
    /// * `Ok(None)` - Si aucun utilisateur n'est trouvé
    /// * `Err(DbErr)` - En cas d'erreur de base de données
    ///
    /// # Exemple
    ///
    /// ```rust
    /// let user = UserRepository::find_by_email(&db, "user@example.com").await?;
    /// match user {
    ///     Some(u) => println!("User found: {}", u.email),
    ///     None => println!("User not found"),
    /// }
    /// ```
    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<User>, DbErr> {
        // ...
    }
}
```

## Exemple 4 : Module Documentation

```rust
//! Module d'authentification et de gestion des utilisateurs.
//!
//! Ce module fournit :
//! - Les handlers API pour l'inscription, connexion, et gestion de profil
//! - La génération et validation des tokens JWT
//! - Le hashage et la vérification des mots de passe avec bcrypt
//!
//! # Architecture
//!
//! Le module est organisé en plusieurs sous-modules :
//! - `handlers.rs` : Handlers Axum pour les endpoints API
//! - `jwt.rs` : Gestion des tokens JWT (création, validation, refresh)
//! - `password.rs` : Hashage et vérification des mots de passe
//!
//! # Sécurité
//!
//! - Les mots de passe sont hashés avec bcrypt (cost factor: 12)
//! - Les tokens JWT utilisent HS256 avec un secret configurable
//! - Les tokens d'accès expirent après 15 minutes
//! - Les tokens de refresh expirent après 7 jours

pub mod handlers;
pub mod jwt;
pub mod password;
```

## Exemple 5 : Entity avec Sea-ORM

```rust
/// Représente un lien raccourci dans la base de données.
///
/// Cette entité correspond à la table `shortened_links` et contient
/// toutes les informations nécessaires pour gérer un lien raccourci,
/// y compris son code court, l'URL originale, et les métadonnées.
///
/// # Relations
///
/// - Appartient à un `Workspace` (via `workspace_id`)
/// - Appartient à un `User` (via `user_id`)
/// - A plusieurs `LinkClick` (statistiques de clics)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "shortened_links")]
pub struct Model {
    /// Identifiant unique du lien (UUID)
    #[sea_orm(primary_key)]
    pub id: String,
    
    /// Code court utilisé dans l'URL raccourcie (ex: "abc123")
    /// 
    /// Ce code est unique dans toute la base de données et est utilisé
    /// pour rediriger vers l'URL originale.
    pub short_code: String,
    
    /// URL originale à raccourcir
    pub original_url: String,
    
    /// ID du workspace auquel appartient ce lien
    pub workspace_id: String,
    
    /// ID de l'utilisateur qui a créé ce lien
    pub user_id: String,
    
    /// Titre optionnel pour identifier le lien
    pub title: Option<String>,
    
    /// Date de création du lien
    pub created_at: Option<DateTime>,
    
    /// Date d'expiration optionnelle du lien
    /// 
    /// Si `None`, le lien n'expire jamais.
    /// Si une date est définie, le lien devient inactif après cette date.
    pub expires_at: Option<DateTime>,
    
    /// Indique si le lien est actif ou non
    /// 
    /// Un lien inactif ne peut pas être utilisé pour rediriger,
    /// même s'il n'a pas expiré.
    pub is_active: bool,
}
```

## Exemple 6 : Fonction Utilitaire

```rust
/// Génère un code court unique pour un lien raccourci.
///
/// Le code est généré de manière aléatoire et vérifié pour s'assurer
/// qu'il n'existe pas déjà dans la base de données.
///
/// # Arguments
///
/// * `db` - Connexion à la base de données pour vérifier l'unicité
/// * `length` - Longueur du code à générer (par défaut: 6)
///
/// # Returns
///
/// Un code court unique sous forme de `String`
///
/// # Panics
///
/// Cette fonction peut paniquer si elle ne parvient pas à générer
/// un code unique après 100 tentatives.
///
/// # Exemple
///
/// ```rust
/// let code = generate_unique_short_code(&db, 6).await;
/// println!("Generated code: {}", code); // Ex: "aB3xY9"
/// ```
pub async fn generate_unique_short_code(
    db: &DatabaseConnection,
    length: usize,
) -> String {
    // ...
}
```

## Exemple 7 : Configuration

```rust
/// Configuration de l'état de l'application.
///
/// Cette structure contient les dépendances partagées entre
/// les handlers Axum, notamment la connexion à la base de données
/// et la configuration JWT pour la génération de tokens.
///
/// # Thread Safety
///
/// Cette structure est conçue pour être partagée entre plusieurs
/// handlers via `Extension<AppState>` dans Axum.
///
/// # Exemple
///
/// ```rust
/// let db = connect().await?;
/// let jwt_secret = std::env::var("JWT_SECRET")?;
/// let app_state = AppState::new(db, &jwt_secret);
/// ```
pub struct AppState {
    /// Connexion à la base de données (SQLite ou PostgreSQL)
    pub db: Db,
    
    /// Configuration pour la génération des tokens JWT
    pub jwt_config: JwtConfig,
}
```

## Commandes pour Générer la Documentation

```bash
# Générer la documentation
cargo doc

# Générer et ouvrir dans le navigateur
cargo doc --open

# Générer pour toutes les dépendances
cargo doc --all

# Tester les exemples dans la documentation
cargo test --doc
```

La documentation générée sera disponible dans `target/doc/shrtnr/index.html`.

