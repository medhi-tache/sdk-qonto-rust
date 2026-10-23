
use std::env;

use crate::QontoError;

/// URL officielle de base de l'API Qonto.
const DEFAULT_BASE_URL: &str = "https://thirdparty.qonto.com/v2";

/// Mode d'authentification du client Qonto.
#[derive(Clone)]
pub(crate) enum Authentication {
    /// Authentification API historique.
    Credentials { login: String, secret: String },

    /// Utilisation d'un jeton d'accès OAuth 2.0 déjà obtenu.
    BearerToken { access_token: String },
}

/// Configuration nécessaire pour utiliser le SDK Qonto.
#[derive(Clone)]
pub struct QontoConfig {
    pub(crate) base_url: String,
    pub(crate) authentication: Authentication,
}

impl QontoConfig {
    /// Construit une configuration avec les identifiants API historiques.
    pub fn new(
        login: impl Into<String>,
        secret: impl Into<String>,
    ) -> Result<Self, QontoError> {
        Self::with_base_url(login, secret, DEFAULT_BASE_URL)
    }

    /// Construit une configuration avec une URL personnalisée.
    pub fn with_base_url(
        login: impl Into<String>,
        secret: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Result<Self, QontoError> {
        let login = login.into();
        let secret = secret.into();

        if login.trim().is_empty() {
            return Err(QontoError::Configuration(
                "Le login API Qonto est vide".into(),
            ));
        }

        if secret.trim().is_empty() {
            return Err(QontoError::Configuration(
                "La clé secrète API Qonto est vide".into(),
            ));
        }

        Self::build(
            Authentication::Credentials { login, secret },
            base_url.into(),
        )
    }

    /// Construit une configuration utilisant un jeton OAuth 2.0.
    ///
    /// Le jeton doit avoir été obtenu préalablement auprès de Qonto.
    pub fn with_access_token(
        access_token: impl Into<String>,
    ) -> Result<Self, QontoError> {
        Self::with_access_token_and_base_url(access_token, DEFAULT_BASE_URL)
    }

    /// Construit une configuration OAuth avec une URL personnalisée.
    pub fn with_access_token_and_base_url(
        access_token: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Result<Self, QontoError> {
        let access_token = access_token.into();

        if access_token.trim().is_empty() {
            return Err(QontoError::Configuration(
                "Le jeton OAuth est vide".into(),
            ));
        }

        Self::build(
            Authentication::BearerToken { access_token },
            base_url.into(),
        )
    }

    /// Charge la configuration historique depuis l'environnement.
    ///
    /// Un fichier `.env` doit être chargé séparément si nécessaire.
    pub fn from_env() -> Result<Self, QontoError> {
        let login = env::var("QONTO_LOGIN").map_err(|_| {
            QontoError::Configuration("La variable QONTO_LOGIN est absente".into())
        })?;

        let secret = env::var("QONTO_SECRET").map_err(|_| {
            QontoError::Configuration("La variable QONTO_SECRET est absente".into())
        })?;

        let base_url =
            env::var("QONTO_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

        Self::with_base_url(login, secret, base_url)
    }

    /// Charge un jeton d'accès OAuth depuis l'environnement.
    pub fn from_oauth_env() -> Result<Self, QontoError> {
        let access_token = env::var("QONTO_ACCESS_TOKEN").map_err(|_| {
            QontoError::Configuration(
                "La variable QONTO_ACCESS_TOKEN est absente".into(),
            )
        })?;

        let base_url =
            env::var("QONTO_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

        Self::with_access_token_and_base_url(access_token, base_url)
    }

    fn build(
        authentication: Authentication,
        base_url: String,
    ) -> Result<Self, QontoError> {
        if base_url.trim().is_empty()
            || (!base_url.starts_with("https://")
                && !base_url.starts_with("http://"))
        {
            return Err(QontoError::Configuration(
                "L'URL de base doit être une URL HTTP ou HTTPS valide".into(),
            ));
        }

        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            authentication,
        })
    }
}
