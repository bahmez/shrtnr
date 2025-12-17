//! Module d'authentification frontend.
//!
//! Fournit les pages de connexion et d'inscription, ainsi que les fonctions
//! client pour interagir avec l'API d'authentification.

mod login;
mod register;

use serde::{Deserialize, Serialize};

pub use login::LoginPage;
pub use register::RegisterPage;

/// DTO représentant un utilisateur authentifié.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthUserDto {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub created_at: Option<String>,
}

/// Réponse d'authentification contenant l'utilisateur et les tokens JWT.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthResponseDto {
    pub user: AuthUserDto,
    pub access_token: String,
    pub refresh_token: String,
}

/// Réponse d'erreur de l'API.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

/// Payload pour la requête de connexion.
#[derive(Debug, Clone, Serialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

/// Payload pour la requête d'inscription.
#[derive(Debug, Clone, Serialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

/// Payload pour la mise à jour du profil utilisateur.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateProfilePayload {
    pub name: Option<String>,
    pub email: Option<String>,
}

    /// Module client pour les appels API d'authentification.
    ///
    /// Ce module contient toutes les fonctions pour interagir avec l'API d'authentification
    /// côté client (WASM). Il gère également le stockage des tokens dans localStorage.
    ///
    /// # Note
    ///
    /// Ce module est uniquement disponible avec la feature `hydrate`.
    #[cfg(feature = "hydrate")]
pub mod client {
    use super::{
        ApiErrorResponse, AuthResponseDto, AuthUserDto, LoginPayload, RegisterPayload,
        UpdateProfilePayload,
    };
    use gloo_net::http::Request;
    use serde::Serialize;
    use web_sys::window;

    /// Récupère le localStorage avec gestion d'erreur.
    ///
    /// # Returns
    ///
    /// * `Ok(web_sys::Storage)` - Le localStorage
    /// * `Err(String)` - Si le localStorage n'est pas disponible
    fn local_storage_result() -> Result<web_sys::Storage, String> {
        window()
            .ok_or_else(|| "Fenêtre non disponible".to_string())?
            .local_storage()
            .map_err(|_| "Accès à localStorage impossible".to_string())?
            .ok_or_else(|| "localStorage non supporté".to_string())
    }

    /// Récupère le localStorage de manière optionnelle.
    ///
    /// # Returns
    ///
    /// * `Some(web_sys::Storage)` - Si le localStorage est disponible
    /// * `None` - Si le localStorage n'est pas disponible
    fn local_storage_option() -> Option<web_sys::Storage> {
        window().and_then(|win| win.local_storage().ok().flatten())
    }

    /// Fonction générique pour les requêtes POST d'authentification.
    ///
    /// # Arguments
    ///
    /// * `path` - Chemin de l'endpoint API
    /// * `payload` - Données à envoyer (LoginPayload ou RegisterPayload)
    ///
    /// # Returns
    ///
    /// * `Ok(AuthResponseDto)` - Réponse d'authentification avec tokens
    /// * `Err(String)` - Message d'erreur
    async fn post_auth<T: Serialize>(path: &str, payload: &T) -> Result<AuthResponseDto, String> {
        let response = Request::post(path)
            .header("Content-Type", "application/json")
            .json(payload)
            .map_err(|err| err.to_string())?
            .send()
            .await
            .map_err(|err| err.to_string())?;

        if response.ok() {
            response
                .json::<AuthResponseDto>()
                .await
                .map_err(|err| format!("Réponse invalide: {err}"))
        } else {
            let status = response.status();
            let message = response
                .json::<ApiErrorResponse>()
                .await
                .map(|err| err.error)
                .unwrap_or_else(|_| format!("Une erreur est survenue (code {status})"));
            Err(message)
        }
    }

    /// Envoie une requête de connexion à l'API.
    ///
    /// # Arguments
    ///
    /// * `payload` - Email et mot de passe de l'utilisateur
    ///
    /// # Returns
    ///
    /// * `Ok(AuthResponseDto)` - Utilisateur et tokens JWT
    /// * `Err(String)` - Message d'erreur (email/mot de passe invalide, etc.)
    pub async fn post_login(payload: &LoginPayload) -> Result<AuthResponseDto, String> {
        post_auth("/api/auth/login", payload).await
    }

    /// Envoie une requête d'inscription à l'API.
    ///
    /// # Arguments
    ///
    /// * `payload` - Email, mot de passe et nom (optionnel) du nouvel utilisateur
    ///
    /// # Returns
    ///
    /// * `Ok(AuthResponseDto)` - Utilisateur créé et tokens JWT
    /// * `Err(String)` - Message d'erreur (email déjà utilisé, etc.)
    pub async fn post_register(payload: &RegisterPayload) -> Result<AuthResponseDto, String> {
        post_auth("/api/auth/register", payload).await
    }

    /// Stocke les tokens JWT dans le localStorage.
    ///
    /// # Arguments
    ///
    /// * `access_token` - Token d'accès JWT
    /// * `refresh_token` - Token de rafraîchissement JWT
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Tokens stockés avec succès
    /// * `Err(String)` - Si le localStorage n'est pas disponible
    pub fn store_tokens(access_token: &str, refresh_token: &str) -> Result<(), String> {
        let storage = local_storage_result()?;

        storage
            .set_item("shrtnr_access_token", access_token)
            .map_err(|_| "Impossible de sauvegarder l'access token".to_string())?;
        storage
            .set_item("shrtnr_refresh_token", refresh_token)
            .map_err(|_| "Impossible de sauvegarder le refresh token".to_string())
    }

    /// Récupère les tokens JWT stockés dans le localStorage.
    ///
    /// # Returns
    ///
    /// * `Some((access_token, refresh_token))` - Si les tokens sont présents
    /// * `None` - Si aucun token n'est stocké ou si le localStorage n'est pas disponible
    pub fn get_stored_tokens() -> Option<(String, String)> {
        let storage = local_storage_option()?;
        let access = storage.get_item("shrtnr_access_token").ok().flatten()?;
        let refresh = storage.get_item("shrtnr_refresh_token").ok().flatten()?;
        Some((access, refresh))
    }

    /// Supprime les tokens JWT du localStorage.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Tokens supprimés avec succès
    /// * `Err(String)` - Si le localStorage n'est pas disponible
    pub fn clear_tokens() -> Result<(), String> {
        let storage = local_storage_result()?;
        storage
            .remove_item("shrtnr_access_token")
            .map_err(|_| "Impossible de supprimer l'access token".to_string())?;
        storage
            .remove_item("shrtnr_refresh_token")
            .map_err(|_| "Impossible de supprimer le refresh token".to_string())
    }

    /// Récupère les informations de l'utilisateur actuellement authentifié.
    ///
    /// # Arguments
    ///
    /// * `access_token` - Token d'accès JWT
    ///
    /// # Returns
    ///
    /// * `Ok(AuthUserDto)` - Informations de l'utilisateur
    /// * `Err(String)` - Si le token est invalide ou expiré
    pub async fn fetch_me(access_token: &str) -> Result<AuthUserDto, String> {
        Request::get("/api/auth/me")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json::<AuthUserDto>()
            .await
            .map_err(|err| format!("Réponse invalide: {err}"))
    }

    /// Met à jour le profil de l'utilisateur.
    ///
    /// # Arguments
    ///
    /// * `access_token` - Token d'accès JWT
    /// * `payload` - Nouvelles informations (nom et/ou email)
    ///
    /// # Returns
    ///
    /// * `Ok(AuthUserDto)` - Utilisateur mis à jour
    /// * `Err(String)` - En cas d'erreur (email déjà utilisé, token invalide, etc.)
    pub async fn update_profile(
        access_token: &str,
        payload: &UpdateProfilePayload,
    ) -> Result<AuthUserDto, String> {
        let response = Request::put("/api/auth/profile")
            .header("Authorization", &format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(payload)
            .map_err(|err| err.to_string())?
            .send()
            .await
            .map_err(|err| err.to_string())?;

        if response.ok() {
            response
                .json::<AuthUserDto>()
                .await
                .map_err(|err| format!("Réponse invalide: {err}"))
        } else {
            let status = response.status();
            let message = response
                .json::<ApiErrorResponse>()
                .await
                .map(|err| err.error)
                .unwrap_or_else(|_| format!("Une erreur est survenue (code {status})"));
            Err(message)
        }
    }

    /// Envoie une requête de déconnexion à l'API.
    ///
    /// Invalide le token côté serveur.
    ///
    /// # Arguments
    ///
    /// * `access_token` - Token d'accès JWT à invalider
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Déconnexion réussie
    /// * `Err(String)` - En cas d'erreur
    pub async fn logout_request(access_token: &str) -> Result<(), String> {
        let response = Request::post("/api/auth/logout")
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|err| err.to_string())?;

        if response.ok() {
            Ok(())
        } else {
            let status = response.status();
            let message = response
                .json::<ApiErrorResponse>()
                .await
                .map(|err| err.error)
                .unwrap_or_else(|_| format!("Une erreur est survenue (code {status})"));
            Err(message)
        }
    }
}

#[cfg(feature = "hydrate")]
pub use client::{
    clear_tokens, fetch_me, get_stored_tokens, logout_request, post_login, post_register,
    store_tokens, update_profile,
};
