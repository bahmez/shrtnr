//! Gestion de l'état d'authentification.
//!
//! Ce module fournit un store réactif pour gérer l'état d'authentification
//! de l'utilisateur, incluant les tokens JWT et les informations utilisateur.

use leptos::prelude::*;

use crate::frontend::pages::auth::AuthUserDto;
#[cfg(feature = "hydrate")]
use crate::frontend::pages::auth::{
    clear_tokens, fetch_me, get_stored_tokens, logout_request, store_tokens, update_profile,
    AuthResponseDto,
};

/// Structure contenant les tokens JWT d'authentification.
#[derive(Clone, Debug)]
pub struct AuthTokens {
    /// Token d'accès JWT (durée de vie courte)
    pub access_token: String,
    /// Token de rafraîchissement JWT (durée de vie longue)
    pub refresh_token: String,
}

/// Store réactif pour l'état d'authentification.
///
/// Utilise Leptos signals pour gérer l'état de manière réactive.
/// Les tokens sont persistés dans localStorage côté client.
#[derive(Clone, Copy)]
pub struct AuthStore {
    /// Utilisateur actuellement authentifié (None si non connecté)
    user: RwSignal<Option<AuthUserDto>>,
    /// Tokens JWT (access et refresh)
    #[allow(dead_code)]
    tokens: RwSignal<Option<AuthTokens>>,
    /// Indique si l'initialisation est en cours (vérification des tokens stockés)
    initializing: RwSignal<bool>,
    /// Dernière erreur d'authentification (optionnel)
    #[allow(dead_code)]
    last_error: RwSignal<Option<String>>,
}

impl AuthStore {
    /// Crée une nouvelle instance d'`AuthStore`.
    ///
    /// Initialise tous les signals à leurs valeurs par défaut.
    pub fn new() -> Self {
        Self {
            user: RwSignal::new(None),
            tokens: RwSignal::new(None),
            initializing: RwSignal::new(true),
            last_error: RwSignal::new(None),
        }
    }

    /// Retourne un signal en lecture seule pour l'utilisateur authentifié.
    ///
    /// # Returns
    ///
    /// Signal contenant `Some(AuthUserDto)` si l'utilisateur est connecté, `None` sinon.
    pub fn user(&self) -> ReadSignal<Option<AuthUserDto>> {
        self.user.read_only()
    }

    /// Vérifie si un utilisateur est actuellement authentifié.
    ///
    /// # Returns
    ///
    /// Signal booléen réactif qui est `true` si un utilisateur est connecté.
    pub fn is_authenticated(&self) -> Signal<bool> {
        let user = self.user();
        Signal::derive(move || user.get().is_some())
    }

    /// Retourne un signal indiquant si l'initialisation est en cours.
    ///
    /// Pendant l'initialisation, le store vérifie les tokens stockés dans localStorage.
    pub fn initializing(&self) -> ReadSignal<bool> {
        self.initializing.read_only()
    }

    /// Marque l'initialisation comme terminée.
    ///
    /// À appeler après avoir vérifié les tokens stockés.
    pub fn finish_initializing(&self) {
        self.initializing.set(false);
    }

    /// Retourne un signal pour la dernière erreur d'authentification.
    ///
    /// # Returns
    ///
    /// Signal contenant `Some(String)` en cas d'erreur, `None` sinon.
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

    /// Applique une réponse d'authentification au store.
    ///
    /// Stocke les tokens dans localStorage et met à jour l'état du store.
    ///
    /// # Arguments
    ///
    /// * `response` - Réponse d'authentification contenant les tokens et l'utilisateur
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Succès
    /// * `Err(String)` - En cas d'erreur lors du stockage des tokens
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

    /// Met à jour les informations de l'utilisateur dans le store.
    ///
    /// Utilisé après une mise à jour de profil.
    ///
    /// # Arguments
    ///
    /// * `user` - Nouvelles informations utilisateur
    #[cfg(feature = "hydrate")]
    pub fn update_user(&self, user: AuthUserDto) {
        self.set_user(Some(user));
        self.set_error(None);
    }

    /// Efface la session utilisateur.
    ///
    /// Supprime les tokens du localStorage et réinitialise l'état du store.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Succès
    /// * `Err(String)` - En cas d'erreur lors de la suppression des tokens
    #[cfg(feature = "hydrate")]
    pub fn clear_session(&self) -> Result<(), String> {
        clear_tokens()?;
        self.set_tokens(None);
        self.set_user(None);
        self.set_error(None);
        Ok(())
    }

    /// Initialise le store en vérifiant les tokens stockés dans localStorage.
    ///
    /// Si des tokens sont trouvés, vérifie leur validité en appelant l'API `/api/auth/me`.
    /// Si les tokens sont invalides, efface la session.
    ///
    /// # Note
    ///
    /// Cette fonction est uniquement disponible côté client (feature "hydrate").
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

    /// Récupère le token d'accès actuel.
    ///
    /// # Returns
    ///
    /// * `Some(String)` - Le token d'accès si disponible
    /// * `None` - Si aucun token n'est disponible
    pub fn access_token(&self) -> Option<String> {
        self.tokens.get().map(|tokens| tokens.access_token.clone())
    }

    /// Récupère le token de rafraîchissement actuel.
    ///
    /// # Returns
    ///
    /// * `Some(String)` - Le token de rafraîchissement si disponible
    /// * `None` - Si aucun token n'est disponible
    ///
    /// # Note
    ///
    /// Cette fonction est uniquement disponible côté client (feature "hydrate").
    #[cfg(feature = "hydrate")]
    pub fn refresh_token(&self) -> Option<String> {
        self.tokens.get().map(|tokens| tokens.refresh_token.clone())
    }

    /// Déconnecte l'utilisateur.
    ///
    /// Appelle l'API de déconnexion pour invalider les tokens côté serveur,
    /// puis efface la session locale.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Succès
    /// * `Err(String)` - En cas d'erreur
    ///
    /// # Note
    ///
    /// Cette fonction est uniquement disponible côté client (feature "hydrate").
    #[cfg(feature = "hydrate")]
    pub async fn logout(&self) -> Result<(), String> {
        if let Some(tokens) = self.tokens.get_untracked() {
            let _ = logout_request(&tokens.access_token).await;
        }
        self.clear_session()
    }

    /// Rafraîchit les informations de l'utilisateur depuis l'API.
    ///
    /// Appelle `/api/auth/me` pour récupérer les dernières informations utilisateur.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Succès, le profil a été mis à jour
    /// * `Err(String)` - En cas d'erreur (session expirée, etc.)
    ///
    /// # Note
    ///
    /// Cette fonction est uniquement disponible côté client (feature "hydrate").
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

    /// Met à jour le profil utilisateur via l'API.
    ///
    /// Envoie les modifications de profil à l'API et met à jour le store local.
    ///
    /// # Arguments
    ///
    /// * `name` - Nouveau nom (optionnel, laisse inchangé si None)
    /// * `email` - Nouvel email (optionnel, laisse inchangé si None)
    ///
    /// # Returns
    ///
    /// * `Ok(AuthUserDto)` - L'utilisateur mis à jour
    /// * `Err(String)` - En cas d'erreur (session expirée, email déjà utilisé, etc.)
    ///
    /// # Note
    ///
    /// Cette fonction est uniquement disponible côté client (feature "hydrate").
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

/// Fournit le store d'authentification dans le contexte Leptos.
///
/// Crée un nouveau `AuthStore`, l'ajoute au contexte pour qu'il soit accessible
/// dans tous les composants enfants, et initialise la vérification des tokens
/// stockés dans localStorage (côté client uniquement).
///
/// # Returns
///
/// L'instance du `AuthStore` créée.
///
/// # Note
///
/// Cette fonction doit être appelée une seule fois au niveau racine de l'application
/// (généralement dans le composant `App`).
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

    #[cfg(not(feature = "hydrate"))]
    {
        store.finish_initializing();
    }

    store
}

/// Récupère le store d'authentification depuis le contexte Leptos.
///
/// # Returns
///
/// L'instance du `AuthStore` depuis le contexte.
///
/// # Panics
///
/// Panique si `AuthStore` n'a pas été fourni dans le contexte (si `provide_auth_store()`
/// n'a pas été appelé dans un composant parent).
pub fn use_auth_store() -> AuthStore {
    use_context::<AuthStore>().expect("AuthStore n'a pas été fourni dans le contexte.")
}
