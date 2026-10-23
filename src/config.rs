use std::env;

use crate::QontoError;

/// URL officielle de base de l'API Qonto.
const DEFAULT_BASE_URL: &str = "https://thirdparty.qonto.com/v2";

/// Configuration nécessaire pour utiliser le SDK Qonto.
#[derive(Clone)]
pub struct QontoConfig {
    pub(crate) base_url: String,
    pub(crate) login: String,
    pub(crate) secret: String,
}

impl QontoConfig {
    /// Construit une configuration à partir des identifiants fournis.
    pub fn new(login: impl Into<String>, secret: impl Into<String>) -> Result<Self, QontoError> {
        Self::with_base_url(login, secret, DEFAULT_BASE_URL)
    }

    /// Construit une configuration avec une URL de base personnalisée.
    pub fn with_base_url(
        login: impl Into<String>,
        secret: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Result<Self, QontoError> {
        let login = login.into();
        let secret = secret.into();
        let base_url = base_url.into();

        if login.trim().is_empty() {
            return Err(QontoError::Configuration(
                "Le login API Qonto est vide".to_string(),
            ));
        }

        if secret.trim().is_empty() {
            return Err(QontoError::Configuration(
                "La clé secrète API Qonto est vide".to_string(),
            ));
        }

        if base_url.trim().is_empty() {
            return Err(QontoError::Configuration(
                "L'URL de base de l'API est vide".to_string(),
            ));
        }

        if !base_url.starts_with("https://") && !base_url.starts_with("http://") {
            return Err(QontoError::Configuration(
                "L'URL de base doit commencer par http:// ou https://".to_string(),
            ));
        }

        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            login,
            secret,
        })
    }

    /// Charge la configuration depuis les variables d'environnement.
    ///
    /// L'application doit charger son fichier .env avant cet appel,
    /// si elle en utilise un.
    pub fn from_env() -> Result<Self, QontoError> {
        let login = env::var("QONTO_LOGIN").map_err(|_| {
            QontoError::Configuration("La variable QONTO_LOGIN est absente".to_string())
        })?;

        let secret = env::var("QONTO_SECRET").map_err(|_| {
            QontoError::Configuration("La variable QONTO_SECRET est absente".to_string())
        })?;

        let base_url = env::var("QONTO_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

        Self::with_base_url(login, secret, base_url)
    }
}
