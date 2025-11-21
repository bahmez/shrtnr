use serde::{Deserialize, Serialize};

/// Représente une réponse d'erreur générique renvoyée par l'API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
}

impl ApiError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            error: message.into(),
        }
    }
}

/// Représente une réponse de succès simple contenant un message utilisateur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessage {
    pub message: String,
}

impl ApiMessage {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
