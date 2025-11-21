# shrtnr

Plateforme de raccourcissement d’URL orientée marketing. Le projet expose :

- un backend Rust/Axum (authentification, gestion de liens, workspaces, statistiques) ;
- un frontend Leptos + Tailwind (landing page marketing, dashboard protégé, système d’authentification complet).

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
git clone https://github.com/.../shrtnr.git
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

## Points complémentaires

- **Configuration** : via variables d’environnement (`DATABASE_URL`, `JWT_SECRET`, etc.), voir `src/backend/config.rs`.
- **Authentification** : tokens JWT stockés côté client (`localStorage`) et rafraîchis via le backend.
- **Routes protégées** : tout layout dashboard passe par `DashboardLayout`, qui vérifie la session et redirige vers `/login`.
- **Tailwind** : configuration dans `tailwind.config.cjs`, entrées principales dans `style/tailwind.input.css`.
- **Build WASM** : le profil `wasm-release` (Cargo.toml) optimise la taille pour la prod.

## Ressources

- [Leptos documentation](https://leptos.dev/)
- [Cargo Leptos](https://github.com/leptos-rs/cargo-leptos)
- Scripts migrations & Postman collection disponibles dans les dossiers `migrations/` et `postman/`.

