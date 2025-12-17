#![recursion_limit = "2048"]

//! # shrtnr
//!
//! Plateforme de raccourcissement d'URL orientée marketing.
//!
//! Cette bibliothèque fournit :
//! - Un backend API REST avec Axum pour la gestion des liens, workspaces, et analytics
//! - Un frontend Leptos avec SSR et hydration pour l'interface utilisateur
//! - Des types partagés entre backend et frontend
//!
//! ## Architecture
//!
//! Le projet est organisé en trois modules principaux :
//! - `backend` : API REST, authentification, gestion de données (disponible avec la feature `ssr`)
//! - [`frontend`] : Interface utilisateur Leptos avec composants réutilisables
//! - [`shared`] : Types et structures partagées entre backend et frontend
//!
//! ## Features
//!
//! - `ssr` : Active le serveur Axum et le rendu côté serveur (activé par défaut pour le binaire)
//! - `hydrate` : Active l'hydratation côté client pour le frontend WASM

#[cfg(feature = "ssr")]
pub mod backend;
pub mod frontend;
pub mod shared;

#[cfg(feature = "hydrate")]
/// Fonction d'hydratation pour le frontend WASM.
///
/// Cette fonction est appelée automatiquement par wasm-bindgen lors du chargement
/// du module WASM dans le navigateur. Elle hydrate l'application Leptos
/// en attachant les event listeners et en activant la réactivité.
///
/// # Panics
///
/// Configure le panic hook pour afficher les erreurs dans la console du navigateur.
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::frontend::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
