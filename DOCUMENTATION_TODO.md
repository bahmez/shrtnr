# TODO - Documentation des Fonctions

Ce fichier liste les fonctions publiques qui doivent être documentées avec `///`.

## ✅ Déjà documentées

### Backend
- ✅ `lib.rs` - hydrate()
- ✅ `main.rs` - main()
- ✅ `backend/config.rs` - AppState::new(), JwtConfig::new()
- ✅ `backend/db.rs` - connect()
- ✅ `backend/middleware.rs` - AuthUser (FromRequestParts)
- ✅ `backend/auth/handlers.rs` - Tous les handlers (register, login, logout, refresh, me, update_profile)
- ✅ `backend/auth/jwt.rs` - create_token_pair(), verify_access_token(), verify_refresh_token()
- ✅ `backend/auth/password.rs` - hash_password(), verify_password()
- ✅ `backend/links/handlers.rs` - Tous les handlers (create, list, get, update, delete, redirect)
- ✅ `backend/repositories/user_repository.rs` - Toutes les méthodes

## ⏳ À documenter

### Backend Handlers

#### `backend/workspaces/handlers.rs`
- [ ] create_workspace_handler
- [ ] list_workspaces_handler
- [ ] get_workspace_handler
- [ ] update_workspace_handler
- [ ] delete_workspace_handler
- [ ] add_workspace_member_handler
- [ ] remove_workspace_member_handler
- [ ] list_workspace_members_handler

#### `backend/analytics/handlers.rs`
- [ ] get_analytics_overview_handler
- [ ] get_clicks_by_date_handler
- [ ] get_geographic_breakdown_handler
- [ ] get_referrer_breakdown_handler
- [ ] get_device_breakdown_handler
- [ ] get_link_detailed_analytics_handler

#### `backend/stats/handlers.rs`
- [ ] get_link_stats_handler
- [ ] get_workspace_stats_handler
- [ ] get_dashboard_stats_handler

### Backend Repositories

#### `backend/repositories/workspace_repository.rs`
- [ ] Toutes les méthodes publiques

#### `backend/repositories/workspace_member_repository.rs`
- [ ] Toutes les méthodes publiques

#### `backend/repositories/shortened_link_repository.rs`
- [ ] Toutes les méthodes publiques

#### `backend/repositories/link_click_repository.rs`
- [ ] Toutes les méthodes publiques

### Frontend

#### `frontend/state/auth.rs`
- [ ] provide_auth_store()
- [ ] use_auth_store()
- [ ] Méthodes de AuthStore

#### `frontend/state/workspaces.rs`
- [ ] provide_workspace_store()
- [ ] use_workspace_store()
- [ ] Méthodes de WorkspaceStore

#### `frontend/components/`
- [ ] Navigation component
- [ ] Footer component

#### `frontend/design_system/components/`
- [ ] Button component
- [ ] Input component
- [ ] Card components
- [ ] Badge component
- [ ] Chip component
- [ ] Heading component
- [ ] Text component

## Format de documentation

Pour chaque fonction, utiliser le format suivant :

```rust
/// [Description courte en une ligne]
///
/// [Description détaillée si nécessaire]
///
/// # Arguments
///
/// * `param1` - Description
/// * `param2` - Description
///
/// # Returns
///
/// Description de la valeur de retour
///
/// # Errors
///
/// * `ErrorType` - Description
///
/// # Example
///
/// ```rust
/// // Exemple d'utilisation
/// ```
```

