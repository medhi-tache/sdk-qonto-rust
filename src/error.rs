use reqwest::StatusCode;
use thiserror::Error;

/// Regroupe toutes les erreurs que le SDK Qonto peut retourner.
#[derive(Debug, Error)]
pub enum QontoError {
    /// Erreur lors de la construction ou de l'envoi d'une requête HTTP.
    #[error("Erreur HTTP : {0}")]
    Http(#[from] reqwest::Error),

    /// Identifiants ou paramètres de configuration manquants ou invalides.
    #[error("Erreur de configuration : {0}")]
    Configuration(String),

    /// L'API Qonto a répondu avec un statut HTTP d'erreur.
    #[error("Erreur API Qonto (HTTP {status}) : {message}")]
    Api { status: StatusCode, message: String },

    /// La réponse de l'API ne correspond pas au format JSON attendu.
    #[error("Erreur de désérialisation JSON : {0}")]
    Json(#[from] serde_json::Error),
}
