//! Types de réponse standardisés pour l'API.
//!
//! Ce module définit les structures de réponse communes utilisées
//! par tous les endpoints de l'API pour maintenir une cohérence
//! dans les réponses HTTP.

use serde::{Deserialize, Serialize};

/// Représente une réponse d'erreur générique renvoyée par l'API.
///
/// Toutes les erreurs de l'API utilisent cette structure pour garantir
/// une cohérence dans les réponses d'erreur.
///
/// # Exemple
///
/// ```rust
/// use shrtnr::shared::responses::ApiError;
///
/// let error = ApiError::new("User not found");
/// // Sérialisé en JSON : {"error": "User not found"}
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Message d'erreur descriptif pour l'utilisateur
    pub error: String,
}

impl ApiError {
    /// Crée une nouvelle instance d'`ApiError` avec le message fourni.
    ///
    /// # Arguments
    ///
    /// * `message` - Le message d'erreur (peut être une `String` ou `&str`)
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use shrtnr::shared::responses::ApiError;
    ///
    /// let error = ApiError::new("Invalid credentials");
    /// let error2 = ApiError::new(String::from("Database error"));
    /// ```
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            error: message.into(),
        }
    }
}

/// Représente une réponse de succès simple contenant un message utilisateur.
///
/// Utilisée pour les endpoints qui retournent uniquement un message
/// de confirmation sans données supplémentaires.
///
/// # Exemple
///
/// ```rust
/// use shrtnr::shared::responses::ApiMessage;
///
/// let message = ApiMessage::new("Link deleted successfully");
/// // Sérialisé en JSON : {"message": "Link deleted successfully"}
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessage {
    /// Message de succès à afficher à l'utilisateur
    pub message: String,
}

impl ApiMessage {
    /// Crée une nouvelle instance d'`ApiMessage` avec le message fourni.
    ///
    /// # Arguments
    ///
    /// * `message` - Le message de succès (peut être une `String` ou `&str`)
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use shrtnr::shared::responses::ApiMessage;
    ///
    /// let message = ApiMessage::new("Operation completed");
    /// ```
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
