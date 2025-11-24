#[cfg(feature = "hydrate")]
use crate::frontend::pages::auth::{ApiErrorResponse, AuthUserDto, UpdateProfilePayload};
#[cfg(feature = "hydrate")]
use gloo_net::http::Request;

#[cfg(feature = "hydrate")]
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
