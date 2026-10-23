use reqwest::Method;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;

use crate::config::Authentication;
use crate::{QontoConfig, QontoError};

/// Client HTTP permettant de communiquer avec l'API Qonto.
pub struct QontoClient {
    /// Client HTTP réutilisable pour envoyer les requêtes.
    pub(crate) http: reqwest::Client,

    /// URL de base de l'API Qonto.
    pub(crate) base_url: String,
}

impl QontoClient {
    /// Construit un client Qonto avec le mode d'authentification configuré.
    pub fn new(config: QontoConfig) -> Result<Self, QontoError> {
        let mut headers = HeaderMap::new();

        let authorization_value = match &config.authentication {
            Authentication::Credentials { login, secret } => {
                format!("{login}:{secret}")
            }
            Authentication::BearerToken { access_token } => {
                format!("Bearer {access_token}")
            }
        };

        let mut authorization = HeaderValue::from_str(&authorization_value).map_err(|error| {
            QontoError::Configuration(format!("Valeur d'authentification HTTP invalide : {error}"))
        })?;

        authorization.set_sensitive(true);
        headers.insert(AUTHORIZATION, authorization);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            http,
            base_url: config.base_url,
        })
    }

    /// Envoie une requête HTTP et convertit la réponse JSON en type T.
    pub(crate) async fn request<T>(
        &self,
        method: Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T, QontoError>
    where
        T: DeserializeOwned,
    {
        let url = format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        );

        let mut request = self.http.request(method, url);

        if let Some(json_body) = body {
            request = request.json(&json_body);
        }

        let response = request.send().await?;
        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            let message = if response_text.trim().is_empty() {
                "La réponse de l'API ne contient aucun détail".to_string()
            } else {
                response_text
            };

            return Err(QontoError::Api { status, message });
        }

        Ok(serde_json::from_str::<T>(&response_text)?)
    }
}
