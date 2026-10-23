use reqwest::Method;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;

use crate::{QontoConfig, QontoError};

/// Client HTTP permettant de communiquer avec l'API Qonto.
pub struct QontoClient {
    /// Client HTTP réutilisable pour envoyer les requêtes.
    pub(crate) http: reqwest::Client,

    /// URL de base de l'API Qonto.
    pub(crate) base_url: String,
}

impl QontoClient {
    /// Construit un client Qonto à partir de sa configuration.
    pub fn new(config: QontoConfig) -> Result<Self, QontoError> {
        // Prépare les en-têtes HTTP par défaut.
        let mut headers = HeaderMap::new();

        // Assemble l'identifiant et le secret au format attendu par Qonto.
        let credentials = format!("{}:{}", config.login, config.secret);

        // Convertit les identifiants en valeur d'en-tête HTTP valide.
        let mut authorization = HeaderValue::from_str(&credentials).map_err(|error| {
            QontoError::Configuration(format!(
                "Identifiants invalides pour l'en-tête HTTP : {error}"
            ))
        })?;

        // Marque la valeur comme sensible pour les outils de débogage compatibles.
        authorization.set_sensitive(true);

        // Ajoute l'authentification aux requêtes de ce client.
        headers.insert(AUTHORIZATION, authorization);

        // Construit le client HTTP réutilisable.
        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        // Retourne le client configuré.
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
        // Assemble l'URL complète à partir de l'URL de base et du chemin.
        let url = format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        );

        // Prépare la requête avec la méthode HTTP et l'URL choisies.
        let mut request = self.http.request(method, url);

        // Ajoute un corps JSON uniquement si un corps a été fourni.
        if let Some(json_body) = body {
            request = request.json(&json_body);
        }

        // Envoie la requête et attend la réponse du serveur.
        let response = request.send().await?;

        // Conserve le statut HTTP avant de lire le corps de la réponse.
        let status = response.status();

        // Lit le corps de la réponse sous forme de texte.
        let response_text = response.text().await?;

        // Si le statut indique un échec, retourne une erreur API.
        if !status.is_success() {
            let message = if response_text.trim().is_empty() {
                "La réponse de l'API ne contient aucun détail".to_string()
            } else {
                response_text
            };

            return Err(QontoError::Api { status, message });
        }

        // Convertit le JSON reçu en type T.
        let result = serde_json::from_str::<T>(&response_text)?;

        // Retourne la valeur désérialisée.
        Ok(result)
    }
}
