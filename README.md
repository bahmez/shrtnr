# shrtnr

Plateforme de raccourcissement d'URL orientée marketing. Le projet expose :

- un backend Rust/Axum (authentification, gestion de liens, workspaces, statistiques) ;
- un frontend Leptos + Tailwind (landing page marketing, dashboard protégé, système d'authentification complet).

## Technologies Utilisées

### Backend

- **Rust** (1.75+) : Langage de programmation système, performant et sûr
- **Axum** (0.8.0) : Framework web asynchrone pour Rust, basé sur Tokio
  - Routing, extraction de données, middleware
  - Support des handlers async/await
- **Tokio** (1.x) : Runtime asynchrone pour Rust
  - Features : `rt-multi-thread` pour le multi-threading
- **Sea-ORM** (0.12) : ORM moderne et type-safe pour Rust
  - Support SQLite et PostgreSQL
  - Features : `macros`, `runtime-tokio-rustls`, `sqlx-postgres`, `sqlx-sqlite`
- **jsonwebtoken** (9.x) : Implémentation JWT pour l'authentification
- **bcrypt** (0.15) : Hashage sécurisé des mots de passe
- **serde** + **serde_json** : Sérialisation/désérialisation JSON
- **uuid** (1.x) : Génération d'identifiants uniques
- **chrono** (0.4) : Gestion des dates et heures
- **tower** + **tower-http** : Middleware et utilitaires HTTP
  - CORS support
- **axum-extra** : Extensions pour Axum (typed headers)
- **moka** (0.12) : Cache en mémoire pour optimiser les performances
- **rand** (0.8) : Génération de codes aléatoires pour les liens courts
- **dotenvy** (0.15) : Gestion des variables d'environnement

### Frontend

- **Leptos** (0.8.0) : Framework web full-stack pour Rust
  - SSR (Server-Side Rendering) avec Axum
  - Hydration côté client
  - Reactivité fine avec signals
- **Leptos Router** (0.8.0) : Système de routing pour Leptos
- **Leptos Meta** (0.8.0) : Gestion des meta tags, stylesheets, etc.
- **Leptos Axum** (0.8.0) : Intégration Leptos avec Axum pour le SSR
- **Tailwind CSS** (3.4.13) : Framework CSS utility-first
  - Configuration personnalisée dans `tailwind.config.cjs`
  - Compilation via PostCSS et Lightning CSS
- **PostCSS** (8.4.47) : Traitement CSS avec plugins
- **Autoprefixer** (10.4.20) : Ajout automatique des préfixes navigateurs
- **wasm-bindgen** (0.2.105) : Bindings WebAssembly pour le frontend
- **gloo-net** (0.4) : Client HTTP pour le frontend WASM
- **web-sys** (0.3) : Bindings pour les APIs Web (Storage, Window, console)

### Base de Données

- **SQLite** : Base de données par défaut (fichier `shrtnr.db`)
- **PostgreSQL** : Support optionnel via variable d'environnement `DATABASE_URL`
- **Migrations SQL** : Scripts de migration compatibles SQLite et PostgreSQL

### Infrastructure & DevOps

- **Docker** : Containerisation de l'application
  - Build multi-stage (builder + runtime)
  - Image optimisée pour la production
- **Docker Compose** : Orchestration pour le développement local
- **Google Cloud Build** : CI/CD pipeline
  - Build automatique avec cache Docker
  - Déploiement sur Cloud Run
- **Google Cloud Run** : Plateforme serverless pour le déploiement
- **cargo-leptos** : Outil CLI pour gérer les projets Leptos
  - Build, serve, hot reload
  - Génération des assets WASM

### Outils de Développement

- **Postman** : Collections de tests API
  - Collections pour auth, workspaces, stats
  - Environnements de développement et production
  - Scripts de test automatisés
- **Node.js** + **npm** : Gestion des dépendances frontend (Tailwind)
- **Rust Toolchain** : Compilateur Rust et outils associés
  - `cargo` : Gestionnaire de paquets et build
  - `rustfmt` : Formatage du code
  - `clippy` : Linter Rust

### Architecture

- **Pattern Repository** : Abstraction de l'accès aux données
- **Handlers Axum** : Gestion des endpoints API
- **Middleware** : Authentification, CORS, gestion d'erreurs
- **Design System** : Composants réutilisables (Button, Card, Input, etc.)
- **State Management** : Signals Leptos pour la réactivité
- **SSR + Hydration** : Rendu serveur avec hydratation côté client

## Structure rapide

```
src/
  backend/       # API Axum, repositories, handlers
  frontend/      # composants, pages Leptos, layouts
  shared/        # types de réponse partagés
style/           # sources CSS/Tailwind
migrations/      # scripts SQL
```

## Prérequis

- **Rust** 1.75+ (toolchain stable recommandée)
- **cargo-leptos** (`cargo install cargo-leptos`)
- **Node.js** + **npm** (pour Tailwind / build CSS)
- Base de données (SQLite par défaut via `shrtnr.db`, PostgreSQL possible via `DATABASE_URL`)

## Installation

```bash
git clone https://github.com/bahmez/shrtnr.git
cd shrtnr
cargo fetch
npm install
```

## Migrations base de données

Le projet fournit des scripts multiplateformes :

- Linux/macOS :

```bash
./migrations/run_migration.sh
```

- Windows PowerShell :

```powershell
.\migrations\run_migration.ps1
```

Par défaut, une base SQLite `shrtnr.db` est créée à la racine. Pour PostgreSQL, définissez `DATABASE_URL` avant d’exécuter la migration.

## Build & tests CSS (Tailwind)

- Construire le CSS en one-shot :

```bash
npm run tailwind:build
```

- Lancer Tailwind en mode watch (rebuild automatique) :

```bash
npm run tailwind:watch
```

> **Important** : exécuter `npm install` une fois avant d’utiliser ces commandes.

## Développement (mode SSR + Hot Reload)

Le projet s’appuie sur `cargo-leptos` :

```bash
cargo leptos serve
```

Comportement :
- backend Axum sur `http://127.0.0.1:3000`
- frontend Leptos rendu côté serveur + hydration côté client
- Tailwind : lancer en parallèle `npm run tailwind:watch` pour rafraîchir les styles

## Build production

1. Générer le CSS :

```bash
npm run tailwind:build
```

2. Construire les artefacts WASM + serveur :

```bash
cargo leptos build --release
```

Le build produit :
- `target/site/` : assets statiques (CSS, JS, WASM)
- `target/release/shrtnr` : binaire Axum compilé (serveur SSR)

3. Lancement (avec env `LEPTOS_ENV=PROD` si nécessaire) :

```bash
LEPTOS_ENV=PROD ./target/release/shrtnr
```

## Tests

Actuellement pas de batterie de tests automatisés fournie.
Quelques checks utiles :

- **Rust** : `cargo check`, `cargo fmt -- --check`, `cargo clippy`
- **Frontend** : `npm run tailwind:build` (vérifie la pipeline Tailwind/Lightning CSS)

## Documentation du Code

Le projet utilise la documentation Rust standard avec `cargo doc`.

### Générer la Documentation

```bash
# Générer et ouvrir la documentation dans le navigateur
cargo doc --open

# Générer la documentation sans l'ouvrir
cargo doc

# Tester les exemples dans la documentation
cargo test --doc
```

La documentation générée est accessible dans `target/doc/shrtnr/index.html`.

### Guide de Documentation

Pour apprendre à documenter le code Rust :

- **Guide complet** : Voir [`DOCUMENTATION.md`](DOCUMENTATION.md) pour les instructions détaillées
- **Exemples concrets** : Voir [`EXEMPLE_DOCUMENTATION.md`](EXEMPLE_DOCUMENTATION.md) pour des exemples de documentation

### Format de Documentation

Utilisez `///` pour documenter les items (fonctions, structs, etc.) :

```rust
/// Inscrit un nouvel utilisateur dans le système.
///
/// # Arguments
///
/// * `state` - L'état de l'application
/// * `payload` - Les données d'inscription
///
/// # Returns
///
/// Une `AuthResponse` avec les tokens JWT
pub async fn register_handler(...) { }
```

Utilisez `//!` pour documenter un module (au début du fichier) :

```rust
//! Module d'authentification.
//!
//! Ce module gère l'inscription, la connexion et la gestion des tokens JWT.
```

## Points complémentaires

- **Configuration** : via variables d’environnement (`DATABASE_URL`, `JWT_SECRET`, etc.), voir `src/backend/config.rs`.
- **Authentification** : tokens JWT stockés côté client (`localStorage`) et rafraîchis via le backend.
- **Routes protégées** : tout layout dashboard passe par `DashboardLayout`, qui vérifie la session et redirige vers `/login`.
- **Tailwind** : configuration dans `tailwind.config.cjs`, entrées principales dans `style/tailwind.input.css`.
- **Build WASM** : le profil `wasm-release` (Cargo.toml) optimise la taille pour la prod.

## Architecture Technique

### Structure du Projet

```
shrtnr/
├── src/
│   ├── backend/          # Backend API
│   │   ├── auth/         # Authentification JWT
│   │   ├── links/        # Gestion des liens raccourcis
│   │   ├── workspaces/   # Gestion des espaces de travail
│   │   ├── analytics/    # Analytics et statistiques
│   │   ├── stats/        # Statistiques dashboard
│   │   ├── entities/     # Modèles de données Sea-ORM
│   │   ├── repositories/ # Pattern Repository
│   │   ├── middleware.rs # Middleware Axum
│   │   ├── config.rs     # Configuration
│   │   └── db.rs         # Connexion base de données
│   ├── frontend/         # Frontend Leptos
│   │   ├── pages/        # Pages de l'application
│   │   ├── components/   # Composants réutilisables
│   │   ├── layouts/      # Layouts (dashboard, etc.)
│   │   ├── design_system/# Système de design
│   │   └── state/        # State management
│   ├── shared/           # Types partagés
│   ├── main.rs           # Point d'entrée serveur
│   └── lib.rs            # Point d'entrée bibliothèque
├── migrations/           # Scripts SQL de migration
├── postman/              # Collections Postman
├── style/                # Sources CSS/Tailwind
├── public/               # Assets statiques
├── Dockerfile            # Image Docker
├── docker-compose.yml    # Compose pour développement
├── cloudbuild.yaml       # Pipeline CI/CD Google Cloud
└── Cargo.toml            # Configuration Rust
```

### Fonctionnalités Principales

#### Backend API

- **Authentification** : Register, Login, Logout, Refresh Token, Profile
- **Workspaces** : CRUD complet, gestion des membres, rôles (owner/admin/member)
- **Liens** : Création, modification, suppression, redirection avec tracking
- **Analytics** : 
  - Vue d'ensemble (total, today, week, month, top links)
  - Clicks par date
  - Breakdown géographique
  - Breakdown par device
  - Breakdown par referrer
  - Analytics détaillés par lien
- **Statistiques** : Dashboard stats, stats par lien, stats par workspace

#### Frontend

- **Landing Page** : Page marketing complète (Hero, Features, Workflow, Pricing, Testimonials, CTA)
- **Authentification** : Pages login et register avec validation
- **Dashboard** : Interface protégée avec navigation
- **Workspaces** : Gestion des espaces de travail
- **Links** : Gestion complète des liens raccourcis
- **Analytics** : Visualisations et graphiques de données
- **Settings** : Paramètres utilisateur

### Sécurité

- **JWT** : Tokens d'authentification avec refresh automatique
- **bcrypt** : Hashage sécurisé des mots de passe
- **CORS** : Configuration des origines autorisées
- **Validation** : Validation des entrées utilisateur
- **Permissions** : Vérification des droits d'accès par workspace

### Performance

- **Cache** : Cache en mémoire (Moka) pour les statistiques fréquentes
- **Index SQL** : Index optimisés pour les requêtes fréquentes
- **WASM** : Build optimisé pour le frontend (profil `wasm-release`)
- **SSR** : Rendu serveur pour un chargement initial rapide

## Ressources

### Documentation Officielle

- [Leptos documentation](https://leptos.dev/)
- [Cargo Leptos](https://github.com/leptos-rs/cargo-leptos)
- [Axum documentation](https://docs.rs/axum/)
- [Sea-ORM documentation](https://www.sea-ql.org/SeaORM/)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### Outils de Formation

- [JetBrains RustRover](https://www.jetbrains.com/rust/) - IDE Rust avec support complet
- [Rustlings](https://github.com/rust-lang/rustlings) - Exercices interactifs Rust

### Scripts et Collections

- Scripts migrations disponibles dans `migrations/`
- Collections Postman disponibles dans `postman/`
  - `auth.collection.json` : Tests d'authentification
  - `workspaces.collection.json` : Tests des workspaces
  - `stats.collection.json` : Tests des statistiques

