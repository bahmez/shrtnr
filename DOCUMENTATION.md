# Documentation du Code

Ce document explique comment documenter le code Rust du projet shrtnr.

## Génération de la Documentation

Pour générer la documentation HTML à partir des commentaires de code :

```bash
cargo doc --open
```

Cette commande :
- Génère la documentation HTML à partir des commentaires `///` et `//!`
- Ouvre automatiquement la documentation dans le navigateur
- La documentation est accessible dans `target/doc/`

Pour générer la documentation sans l'ouvrir :

```bash
cargo doc
```

## Format de Documentation Rust

### Documentation d'un Item (fonction, struct, etc.)

Utilisez `///` pour documenter l'item qui suit :

```rust
/// Authentifie un utilisateur avec son email et mot de passe.
///
/// # Arguments
///
/// * `email` - L'adresse email de l'utilisateur
/// * `password` - Le mot de passe en clair
///
/// # Returns
///
/// Un `Result` contenant soit :
/// - `Ok(AuthResponse)` avec les tokens JWT et les infos utilisateur
/// - `Err(ApiError)` en cas d'échec (mauvais credentials, erreur serveur, etc.)
///
/// # Exemple
///
/// ```rust
/// let response = login_handler(Extension(db), Json(request)).await?;
/// ```
pub async fn login_handler(
    Extension(db): Extension<Db>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    // ...
}
```

### Documentation d'un Module/Crate

Utilisez `//!` pour documenter le module lui-même (au début du fichier) :

```rust
//! Module d'authentification
//!
//! Ce module gère toute l'authentification de l'application :
//! - Inscription et connexion des utilisateurs
//! - Génération et validation des tokens JWT
//! - Hashage et vérification des mots de passe
//! - Gestion des sessions utilisateur

pub mod handlers;
pub mod jwt;
pub mod password;
```

### Documentation d'une Struct

```rust
/// Représente un utilisateur authentifié dans le système.
///
/// Cette structure contient les informations de base d'un utilisateur
/// et est utilisée pour les réponses API.
///
/// # Exemple
///
/// ```rust
/// let user = UserResponse {
///     id: "123".to_string(),
///     email: "user@example.com".to_string(),
///     name: Some("John Doe".to_string()),
///     created_at: Some("2024-01-01T00:00:00Z".to_string()),
/// };
/// ```
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// Identifiant unique de l'utilisateur (UUID)
    pub id: String,
    /// Adresse email de l'utilisateur
    pub email: String,
    /// Nom complet de l'utilisateur (optionnel)
    pub name: Option<String>,
    /// Date de création du compte (ISO 8601)
    pub created_at: Option<String>,
}
```

### Documentation d'un Enum

```rust
/// Types d'erreurs possibles pour l'API.
///
/// Chaque variant représente un type d'erreur spécifique
/// avec un code HTTP et un message approprié.
#[derive(Debug, Serialize)]
pub enum ApiError {
    /// Erreur d'authentification (401)
    Unauthorized(String),
    /// Ressource non trouvée (404)
    NotFound(String),
    /// Erreur de validation (400)
    BadRequest(String),
    /// Erreur serveur interne (500)
    InternalServerError(String),
}
```

### Sections Spéciales

Rust supporte plusieurs sections spéciales dans la documentation :

- `# Arguments` - Liste les paramètres d'une fonction
- `# Returns` - Décrit la valeur de retour
- `# Panics` - Décrit quand la fonction peut paniquer
- `# Errors` - Décrit les erreurs possibles
- `# Safety` - Pour les fonctions `unsafe`
- `# Examples` - Exemples d'utilisation
- `# See also` - Références vers d'autres items

### Exemples de Code

Les exemples dans la documentation sont automatiquement testés avec `cargo test --doc` :

```rust
/// Calcule la somme de deux nombres.
///
/// # Exemple
///
/// ```
/// use shrtnr::utils::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Bonnes Pratiques

### 1. Documenter les Items Publics

Tous les items publics (fonctions, structs, enums, traits, modules) doivent être documentés :

```rust
/// Handler pour créer un nouveau lien raccourci.
pub async fn create_link_handler(...) { }
```

### 2. Documenter les Items Privés Importants

Documentez aussi les fonctions privées complexes qui sont importantes pour comprendre le code :

```rust
/// Génère un code court unique pour un lien.
/// 
/// Utilise un algorithme de génération aléatoire avec vérification
/// d'unicité dans la base de données.
fn generate_short_code() -> String { }
```

### 3. Utiliser Markdown

La documentation Rust supporte Markdown, utilisez-le pour formater :

```rust
/// Crée un nouveau workspace.
///
/// **Note:** Seul le propriétaire peut créer des workspaces.
///
/// # Arguments
///
/// - `name`: Le nom du workspace (requis, max 100 caractères)
/// - `owner_id`: L'ID de l'utilisateur propriétaire
///
/// # Returns
///
/// Le workspace créé avec son ID généré.
pub async fn create_workspace(...) { }
```

### 4. Documenter les Types Complexes

Pour les types complexes, expliquez leur usage et leur structure :

```rust
/// Configuration de l'état de l'application.
///
/// Cette structure contient les dépendances partagées entre
/// les handlers Axum, notamment la connexion à la base de données
/// et le secret JWT pour la génération de tokens.
///
/// # Exemple
///
/// ```rust
/// let app_state = AppState::new(db, "secret-key");
/// ```
pub struct AppState {
    /// Connexion à la base de données
    pub db: Db,
    /// Secret pour la génération des tokens JWT
    pub jwt_secret: String,
}
```

### 5. Liens vers d'Autres Items

Utilisez des liens vers d'autres items avec la syntaxe `[Type]` ou `[crate::module::Item]` :

```rust
/// Crée un nouveau lien raccourci.
///
/// Voir aussi [`ShortenedLink`](crate::backend::entities::shortened_link::ShortenedLink)
/// pour la structure de données.
pub async fn create_link(...) { }
```

## Structure de Documentation Recommandée

### Pour les Handlers API

```rust
/// [Description courte en une ligne]
///
/// [Description détaillée si nécessaire]
///
/// # Arguments
///
/// * `param1` - Description du paramètre
/// * `param2` - Description du paramètre
///
/// # Returns
///
/// Description de la valeur de retour
///
/// # Errors
///
/// * `ApiError::BadRequest` - Si les données sont invalides
/// * `ApiError::Unauthorized` - Si l'utilisateur n'est pas authentifié
///
/// # Exemple
///
/// ```rust
/// // Exemple d'utilisation
/// ```
pub async fn handler_name(...) { }
```

### Pour les Repositories

```rust
/// Repository pour la gestion des utilisateurs.
///
/// Fournit des méthodes CRUD et des requêtes spécialisées
/// pour l'entité User dans la base de données.
pub struct UserRepository;

impl UserRepository {
    /// Trouve un utilisateur par son email.
    ///
    /// # Arguments
    ///
    /// * `db` - Connexion à la base de données
    /// * `email` - L'adresse email à rechercher
    ///
    /// # Returns
    ///
    /// `Option<User>` - L'utilisateur trouvé ou `None`
    pub async fn find_by_email(...) { }
}
```

### Pour les Entities

```rust
/// Représente un lien raccourci dans la base de données.
///
/// Cette entité correspond à la table `shortened_links`
/// et contient toutes les informations nécessaires pour
/// gérer un lien raccourci.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "shortened_links")]
pub struct Model {
    /// Identifiant unique du lien
    #[sea_orm(primary_key)]
    pub id: String,
    /// Code court utilisé dans l'URL (ex: "abc123")
    pub short_code: String,
    /// URL originale à raccourcir
    pub original_url: String,
    // ...
}
```

## Commandes Utiles

```bash
# Générer la documentation
cargo doc

# Générer et ouvrir la documentation
cargo doc --open

# Générer la documentation pour toutes les dépendances
cargo doc --all

# Tester les exemples dans la documentation
cargo test --doc

# Vérifier que la documentation compile sans warnings
cargo doc --no-deps
```

## Documentation en Ligne

Une fois générée, la documentation est accessible localement dans `target/doc/shrtnr/index.html`.

Pour la partager, vous pouvez :
- Déployer sur GitHub Pages
- Utiliser `docs.rs` pour les crates publiés
- Héberger sur un serveur web

## Ressources

- [Rust Book - Documentation](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)
- [Rust API Guidelines - Documentation](https://rust-lang.github.io/api-guidelines/documentation.html)
- [cargo doc Documentation](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)

