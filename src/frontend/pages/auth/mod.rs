mod login;
mod register;

use serde::{Deserialize, Serialize};

pub use login::LoginPage;
pub use register::RegisterPage;
#[derive(Debug, Clone, Deserialize)]
pub struct AuthUserDto {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthResponseDto {
    pub user: AuthUserDto,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateProfilePayload {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[cfg(feature = "hydrate")]
pub mod client {
    use super::{
        ApiErrorResponse, AuthResponseDto, AuthUserDto, LoginPayload, RegisterPayload,
        UpdateProfilePayload,
    };
    use gloo_net::http::Request;
    use serde::Serialize;
    use web_sys::window;

    fn local_storage_result() -> Result<web_sys::Storage, String> {
        window()
            .ok_or_else(|| "Fenêtre non disponible".to_string())?
            .local_storage()
            .map_err(|_| "Accès à localStorage impossible".to_string())?
            .ok_or_else(|| "localStorage non supporté".to_string())
    }

    fn local_storage_option() -> Option<web_sys::Storage> {
        window().and_then(|win| win.local_storage().ok().flatten())
    }

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

    pub async fn post_login(payload: &LoginPayload) -> Result<AuthResponseDto, String> {
        post_auth("/api/auth/login", payload).await
    }

    pub async fn post_register(payload: &RegisterPayload) -> Result<AuthResponseDto, String> {
        post_auth("/api/auth/register", payload).await
    }

    pub fn store_tokens(access_token: &str, refresh_token: &str) -> Result<(), String> {
        let storage = local_storage_result()?;

        storage
            .set_item("shrtnr_access_token", access_token)
            .map_err(|_| "Impossible de sauvegarder l'access token".to_string())?;
        storage
            .set_item("shrtnr_refresh_token", refresh_token)
            .map_err(|_| "Impossible de sauvegarder le refresh token".to_string())
    }

    pub fn get_stored_tokens() -> Option<(String, String)> {
        let storage = local_storage_option()?;
        let access = storage.get_item("shrtnr_access_token").ok().flatten()?;
        let refresh = storage.get_item("shrtnr_refresh_token").ok().flatten()?;
        Some((access, refresh))
    }

    pub fn clear_tokens() -> Result<(), String> {
        let storage = local_storage_result()?;
        storage
            .remove_item("shrtnr_access_token")
            .map_err(|_| "Impossible de supprimer l'access token".to_string())?;
        storage
            .remove_item("shrtnr_refresh_token")
            .map_err(|_| "Impossible de supprimer le refresh token".to_string())
    }

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
