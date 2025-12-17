# Cahier des Charges - shrtnr

## Plateforme de Raccourcissement d'URL Orientée Marketing

**Équipe :** Younes, Adil, Théophilius  
**Durée totale estimée :** 210-240 heures (70-80h par développeur)  
**Date de début :** 22 octobre 2025

---

## 1. Formation et Montée en Compétence

### 1.1 Temps de Formation Initiale
**Estimation : 12-18 heures par développeur**

Étant donné que Rust et Leptos sont des technologies nouvelles pour l'équipe, un temps significatif a été alloué à la formation :

- **Younes et Adil :** Formation via JetBrains RustRover
  - Ressource : [JetBrains RustRover](https://www.jetbrains.com/rust/)
  - Temps estimé : 12-18h
  - Contenu : Syntaxe Rust, ownership, lifetimes, async/await, écosystème Rust

- **Théophilius :** Formation via documentation officielle et Rustlings
  - Ressources :
    - [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
    - [Rustlings](https://github.com/rust-lang/rustlings)
  - Temps estimé : 12-18h
  - Contenu : Fondamentaux Rust, exercices pratiques, patterns courants

**Justification :** La courbe d'apprentissage de Rust est significative, notamment pour les concepts d'ownership, de borrowing et de lifetimes. La formation était essentielle avant de commencer le développement.

---

## 2. Architecture et Conception

### 2.1 Réflexion sur l'Architecture
**Estimation : 5 heures (1.7h par développeur)**

- Analyse des besoins fonctionnels et non-fonctionnels
- Choix de l'architecture full-stack Rust (Leptos + Axum)
- Décision sur la séparation backend/frontend/shared
- Choix de l'ORM (Sea-ORM) et de la base de données (SQLite/PostgreSQL)
- Architecture des repositories et handlers
- Design du système d'authentification JWT
- Structure des workspaces et permissions

**Justification :** Une architecture solide est la base d'un projet maintenable. Le temps investi ici évite les refactorisations plus tard.

### 2.2 Organisation du Projet
**Estimation : 3 heures (1h par développeur)**

- Structure des modules Rust
- Organisation des composants frontend (design system, layouts, pages)
- Configuration Cargo.toml avec features (ssr, hydrate)
- Configuration Tailwind CSS
- Scripts de migration SQL
- Configuration Docker et CI/CD
- Organisation des tests Postman

**Justification :** Une bonne organisation facilite la collaboration et la maintenabilité du code.

---

## 3. Backend - API REST

### 3.1 Configuration et Infrastructure de Base
**Estimation : 6 heures (avec +20% doc = 7.2h)**

- Configuration Axum et routing
- Configuration Sea-ORM avec support SQLite/PostgreSQL
- Gestion des variables d'environnement
- Middleware CORS et authentification
- Gestion des erreurs et réponses API standardisées
- Configuration du state partagé (AppState)

**Temps réel : 7h** (6h développement + 1h documentation)

### 3.2 Système d'Authentification
**Estimation : 10 heures (avec +20% doc = 12h)**

- Implémentation JWT (génération, validation, refresh)
- Hashage de mots de passe avec bcrypt
- Handlers : register, login, logout, refresh-token, me, update-profile
- Middleware d'authentification
- Gestion des tokens dans le state frontend
- Protection des routes API

**Temps réel : 10h** (8h développement + 2h documentation)

### 3.3 Gestion des Workspaces
**Estimation : 10 heures (avec +20% doc = 12h)**

- Modèle de données (workspaces, workspace_members)
- Repository pattern pour les workspaces
- Handlers CRUD : create, read, update, delete
- Gestion des membres (add, remove, list)
- Système de rôles (owner, admin, member)
- Vérification des permissions

**Temps réel : 10h** (8h développement + 2h documentation)

### 3.4 Gestion des Liens Raccourcis
**Estimation : 10 heures (avec +20% doc = 12h)**

- Modèle de données (shortened_links)
- Génération de codes courts uniques
- Handlers CRUD complets
- Redirection avec tracking des clics
- Gestion de l'expiration des liens
- Validation des URLs
- Association aux workspaces

**Temps réel : 10h** (8h développement + 2h documentation)

### 3.5 Système de Tracking et Analytics
**Estimation : 10 heures (avec +20% doc = 12h)**

- Modèle de données (link_clicks)
- Enregistrement des clics (IP, user-agent, referer)
- Géolocalisation des clics (country, city)
- Handlers analytics :
  - Overview (total, today, week, month, top links)
  - Clicks par date (agrégation temporelle)
  - Breakdown géographique
  - Breakdown par device
  - Breakdown par referrer
  - Analytics détaillés par lien
- Optimisation des requêtes SQL pour les agrégations
- Cache avec Moka pour les statistiques fréquentes

**Temps réel : 10h** (8h développement + 2h documentation)

### 3.6 Statistiques Dashboard
**Estimation : 6 heures (avec +20% doc = 7.2h)**

- Handler dashboard stats (vue d'ensemble)
- Stats par lien
- Stats par workspace
- Agrégations complexes SQL
- Optimisation des performances

**Temps réel : 7h** (6h développement + 1h documentation)

---

## 4. Frontend - Interface Utilisateur

### 4.1 Configuration Leptos et Build System
**Estimation : 5 heures (avec +20% doc = 6h)**

- Configuration cargo-leptos
- Setup SSR + hydration
- Configuration Tailwind CSS
- Build pipeline WASM
- Hot reload en développement
- Configuration des features (ssr, hydrate)

**Temps réel : 6h** (5h développement + 1h documentation)

### 4.2 Design System
**Estimation : 8 heures (avec +20% doc = 9.6h)**

- Composants de base réutilisables :
  - Button (variants, sizes)
  - Input (text, email, password)
  - Card (header, body)
  - Badge et Chip
  - Heading (niveaux H1-H6)
  - Text (tones, sizes)
- Système de couleurs cohérent
- Responsive design
- Accessibilité de base

**Temps réel : 10h** (8h développement + 2h documentation)

### 4.3 Système de Routing et Navigation
**Estimation : 4 heures (avec +20% doc = 4.8h)**

- Configuration Leptos Router
- Routes publiques et protégées
- Navigation component
- Footer component
- Gestion des redirections

**Temps réel : 5h** (4h développement + 1h documentation)

### 4.4 Pages d'Authentification
**Estimation : 6 heures (avec +20% doc = 7.2h)**

- Page de login (formulaire, validation, erreurs)
- Page d'inscription (formulaire, validation)
- Intégration avec l'API backend
- Gestion du state d'authentification
- Redirections après login/logout
- Messages d'erreur utilisateur

**Temps réel : 7h** (6h développement + 1h documentation)

### 4.5 Landing Page Marketing
**Estimation : 10 heures (avec +20% doc = 12h)**

- Section Hero (CTA principal)
- Section Features (3 features principales)
- Section Workflow (processus d'utilisation)
- Section Metrics (chiffres clés)
- Section Pricing (tarification)
- Section Testimonials (témoignages)
- Section CTA finale
- Design responsive et moderne
- Animations et transitions

**Temps réel : 10h** (8h développement + 2h documentation)

### 4.6 Dashboard Layout
**Estimation : 8 heures (avec +20% doc = 9.6h)**

- Layout principal avec sidebar/navbar
- Navigation entre sections
- Footer du dashboard
- Modals (settings, workspace)
- Protection des routes (redirection si non authentifié)
- Gestion du workspace actif

**Temps réel : 10h** (8h développement + 2h documentation)

### 4.7 Page Workspace
**Estimation : 6 heures (avec +20% doc = 7.2h)**

- Liste des workspaces
- Création de workspace
- Gestion des membres
- Modification/suppression
- Sélection du workspace actif

**Temps réel : 7h** (6h développement + 1h documentation)

### 4.8 Page Links
**Estimation : 8 heures (avec +20% doc = 9.6h)**

- Liste des liens avec filtres
- Création de lien (formulaire complet)
- Édition de lien
- Suppression de lien
- Copie du lien court
- Prévisualisation
- Gestion de l'expiration

**Temps réel : 10h** (8h développement + 2h documentation)

### 4.9 Page Analytics
**Estimation : 10 heures (avec +20% doc = 12h)**

- Vue d'ensemble avec statistiques clés
- Graphiques de clics par date
- Breakdown géographique (carte ou liste)
- Breakdown par device
- Breakdown par referrer
- Analytics détaillés par lien
- Filtres par date
- Sélection de workspace
- Visualisations de données

**Temps réel : 10h** (8h développement + 2h documentation)

### 4.10 Page Settings
**Estimation : 4 heures (avec +20% doc = 4.8h)**

- Modification du profil utilisateur
- Gestion des préférences
- Intégration avec l'API backend

**Temps réel : 5h** (4h développement + 1h documentation)

### 4.11 Pages Secondaires
**Estimation : 3 heures (avec +20% doc = 3.6h)**

- Page Support
- Page Aide
- Page Status
- Intégration dans la navigation

**Temps réel : 4h** (3h développement + 1h documentation)

### 4.12 State Management
**Estimation : 5 heures (avec +20% doc = 6h)**

- Store d'authentification (Leptos signals)
- Store des workspaces
- Synchronisation avec le backend
- Persistence dans localStorage
- Gestion des erreurs

**Temps réel : 6h** (5h développement + 1h documentation)

---

## 5. Base de Données

### 5.1 Modélisation
**Estimation : 2.5 heures (avec +20% doc = 3h)**

- Conception du schéma (5 tables principales)
- Relations entre tables
- Index pour les performances
- Support SQLite et PostgreSQL
- Migration scripts

**Temps réel : 3h** (2.5h développement + 0.5h documentation)

**Justification :** Le schéma est relativement simple avec 5 tables et des relations claires (users, workspaces, workspace_members, shortened_links, link_clicks). La modélisation est rapide car les besoins sont bien définis dès le départ.

### 5.2 Entities et Repositories
**Estimation : 4 heures (avec +20% doc = 4.8h)**

- Définition des entities Sea-ORM (User, Workspace, WorkspaceMember, ShortenedLink, LinkClick)
- Implémentation des repositories avec pattern Repository
- Méthodes CRUD pour chaque entité
- Requêtes complexes pour analytics
- Gestion des transactions

**Temps réel : 5h** (4h développement + 1h documentation)

**Justification :** Sea-ORM génère automatiquement beaucoup de code via ses macros, ce qui accélère significativement le développement des entities. Le pattern Repository est simple à implémenter une fois le modèle compris.

---

## 6. Infrastructure et DevOps

### 6.1 Docker
**Estimation : 2 heures**

- Dockerfile multi-stage (builder + runtime)
- Configuration pour build Rust + Node.js
- Optimisation de la taille de l'image
- Docker Compose pour développement local
- Gestion des variables d'environnement

**Temps réel : 2h**

**Justification :** Les Dockerfiles pour applications Rust/Leptos suivent des patterns standardisés. La configuration est relativement simple une fois le build local fonctionnel.

### 6.2 CI/CD Google Cloud
**Estimation : 3 heures**

- Configuration Cloud Build (cloudbuild.yaml)
- Pipeline de build avec cache Docker
- Déploiement automatique sur Cloud Run
- Gestion des secrets et variables d'environnement
- Configuration des triggers

**Temps réel : 3h**

**Justification :** La configuration Cloud Build est relativement standardisée. Une fois le Dockerfile fonctionnel, le pipeline CI/CD suit des patterns bien documentés.

---

## 7. Tests et Qualité

### 7.1 Tests Postman
**Estimation : 6 heures**

- Collections Postman complètes :
  - Auth collection (tous les endpoints auth)
  - Workspaces collection (CRUD workspaces)
  - Stats collection (endpoints analytics)
- Environnements (dev, production)
- Scripts de test automatisés
- Documentation des endpoints

**Temps réel : 6h**

### 7.2 Tests et Validation
**Estimation : 4 heures**

- Tests manuels de toutes les fonctionnalités
- Validation des formulaires
- Tests de performance basiques
- Vérification de la sécurité (JWT, bcrypt)
- Tests de compatibilité navigateurs

**Temps réel : 4h**

---

## Récapitulatif des Temps

| Catégorie | Temps Estimé (h) |
|-----------|------------------|
| 1. Formation | 36-54h (12-18h × 3) |
| 2. Architecture & Organisation | 8h (5h Architecture + 3h Organisation) |
| 3. Backend | 62h (7h Config + 12h Auth + 12h Workspaces + 12h Links + 12h Analytics + 7h Stats) |
| 4. Frontend | 89h (6h Config + 10h Design + 5h Routing + 7h Auth + 12h Landing + 10h Dashboard + 7h Workspace + 10h Links + 12h Analytics + 5h Settings + 4h Secondaires + 6h State) |
| 5. Base de Données | 8h (3h Modélisation + 5h Entities) |
| 6. Infrastructure & DevOps | 5h (2h Docker + 3h Google Cloud) |
| 7. Tests & Qualité | 10h (6h Postman + 4h Tests) |
| **TOTAL** | **218-236h** |

**Répartition par développeur :** ~73-79h chacun

> **Note importante :** Les temps indiqués incluent déjà un buffer de 20% pour la lecture de documentation et l'apprentissage des technologies Rust/Leptos (sauf pour Docker, Google Cloud, Postman et Tests), étant donné que c'est une nouvelle stack technologique pour l'équipe.

---

## Justification des Estimations

### Facteurs de Complexité

1. **Nouvelle Stack Technologique** : Rust et Leptos nécessitent un temps d'adaptation significatif
2. **Architecture Full-Stack** : Développement backend et frontend dans le même langage mais avec des paradigmes différents
3. **Fonctionnalités Avancées** : Analytics complexes, système de permissions, multi-tenancy
4. **Qualité de Code** : Pattern Repository, séparation des responsabilités, code maintenable
5. **Infrastructure Moderne** : Docker, CI/CD, déploiement cloud

### Points Clés

- Chaque tâche (sauf Docker, Google Cloud, Postman et Tests) inclut 20% de temps supplémentaire pour la documentation et l'apprentissage
- Le temps de formation (15-20h) est comptabilisé séparément au début
- Les sessions de test et validation sont incluses dans chaque module
- La réflexion sur l'architecture et l'organisation est justifiée et nécessaire

---

## Conclusion

Ce projet représente un investissement significatif en temps et en apprentissage pour l'équipe. L'utilisation de Rust et Leptos, bien que bénéfique pour les performances et la maintenabilité, a nécessité une courbe d'apprentissage importante. Le résultat final est une application complète, moderne et performante, avec une architecture solide et une expérience utilisateur soignée.

