use leptos::prelude::*;

use crate::frontend::pages::auth::AuthUserDto;
#[cfg(feature = "hydrate")]
use crate::frontend::pages::auth::{
    clear_tokens, fetch_me, get_stored_tokens, logout_request, store_tokens, update_profile,
    AuthResponseDto,
};

#[derive(Clone, Debug)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone)]
pub struct AuthStore {
    user: RwSignal<Option<AuthUserDto>>,
    #[allow(dead_code)]
    tokens: RwSignal<Option<AuthTokens>>,
    initializing: RwSignal<bool>,
    #[allow(dead_code)]
    last_error: RwSignal<Option<String>>,
}

impl AuthStore {
    pub fn new() -> Self {
        Self {
            user: RwSignal::new(None),
            tokens: RwSignal::new(None),
            initializing: RwSignal::new(true),
            last_error: RwSignal::new(None),
        }
    }

    pub fn user(&self) -> ReadSignal<Option<AuthUserDto>> {
        self.user.read_only()
    }

    pub fn is_authenticated(&self) -> Signal<bool> {
        let user = self.user();
        Signal::derive(move || user.get().is_some())
    }

    pub fn initializing(&self) -> ReadSignal<bool> {
        self.initializing.read_only()
    }

    pub fn last_error(&self) -> ReadSignal<Option<String>> {
        self.last_error.read_only()
    }

    #[allow(dead_code)]
    fn set_user(&self, user: Option<AuthUserDto>) {
        self.user.set(user);
    }

    #[allow(dead_code)]
    fn set_tokens(&self, tokens: Option<AuthTokens>) {
        self.tokens.set(tokens);
    }

    #[allow(dead_code)]
    fn set_error(&self, message: Option<String>) {
        self.last_error.set(message);
    }

    #[cfg(feature = "hydrate")]
    pub fn apply_auth_response(&self, response: &AuthResponseDto) -> Result<(), String> {
        store_tokens(&response.access_token, &response.refresh_token)?;
        self.set_tokens(Some(AuthTokens {
            access_token: response.access_token.clone(),
            refresh_token: response.refresh_token.clone(),
        }));
        self.set_user(Some(response.user.clone()));
        self.set_error(None);
        Ok(())
    }

    #[cfg(feature = "hydrate")]
    pub fn update_user(&self, user: AuthUserDto) {
        self.set_user(Some(user));
        self.set_error(None);
    }

    #[cfg(feature = "hydrate")]
    pub fn clear_session(&self) -> Result<(), String> {
        clear_tokens()?;
        self.set_tokens(None);
        self.set_user(None);
        self.set_error(None);
        Ok(())
    }

    #[cfg(feature = "hydrate")]
    pub async fn initialize(&self) {
        self.initializing.set(true);
        if let Some(tokens) = get_stored_tokens() {
            self.tokens.set(Some(AuthTokens {
                access_token: tokens.0.clone(),
                refresh_token: tokens.1.clone(),
            }));
            match fetch_me(&tokens.0).await {
                Ok(user) => {
                    self.user.set(Some(user));
                    self.last_error.set(None);
                }
                Err(err) => {
                    let _ = self.clear_session();
                    self.last_error.set(Some(err));
                }
            }
        } else {
            self.set_tokens(None);
            self.set_user(None);
        }
        self.initializing.set(false);
    }

    #[cfg(feature = "hydrate")]
    pub fn access_token(&self) -> Option<String> {
        self.tokens.get().map(|tokens| tokens.access_token.clone())
    }

    #[cfg(feature = "hydrate")]
    pub fn refresh_token(&self) -> Option<String> {
        self.tokens.get().map(|tokens| tokens.refresh_token.clone())
    }

    #[cfg(feature = "hydrate")]
    pub async fn logout(&self) -> Result<(), String> {
        if let Some(tokens) = self.tokens.get_untracked() {
            let _ = logout_request(&tokens.access_token).await;
        }
        self.clear_session()
    }

    #[cfg(feature = "hydrate")]
    pub async fn refresh_profile(&self) -> Result<(), String> {
        if let Some(access) = self.access_token() {
            match fetch_me(&access).await {
                Ok(user) => {
                    self.set_user(Some(user));
                    self.set_error(None);
                    Ok(())
                }
                Err(err) => {
                    let _ = self.clear_session();
                    Err(err)
                }
            }
        } else {
            Err("Aucun token disponible".into())
        }
    }

    #[cfg(feature = "hydrate")]
    pub async fn submit_profile(
        &self,
        name: Option<String>,
        email: Option<String>,
    ) -> Result<AuthUserDto, String> {
        if let Some(access) = self.access_token() {
            let payload = crate::frontend::pages::auth::UpdateProfilePayload { name, email };
            match update_profile(&access, &payload).await {
                Ok(user) => {
                    self.update_user(user.clone());
                    Ok(user)
                }
                Err(err) => Err(err),
            }
        } else {
            Err("Session expirée, veuillez vous reconnecter.".into())
        }
    }
}

pub fn provide_auth_store() -> AuthStore {
    let store = AuthStore::new();
    provide_context(store.clone());

    #[cfg(feature = "hydrate")]
    {
        use leptos::task::spawn_local;
        spawn_local({
            let store = store.clone();
            async move {
                store.initialize().await;
            }
        });
    }

    store
}

pub fn use_auth_store() -> AuthStore {
    use_context::<AuthStore>().expect("AuthStore n'a pas été fourni dans le contexte.")
}
