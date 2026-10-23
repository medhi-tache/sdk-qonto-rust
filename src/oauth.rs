use reqwest::Url;
use serde::Deserialize;

use crate::QontoError;

/// Endpoints OAuth 2.0 de Qonto.
const AUTHORIZATION_URL: &str = "https://oauth.qonto.com/oauth2/auth";
const TOKEN_URL: &str = "https://oauth.qonto.com/oauth2/token";

/// Jetons retournés par le serveur OAuth Qonto.
///
/// Ne pas afficher cette structure dans les logs : elle contient des secrets.
#[derive(Deserialize)]
pub struct OAuthTokens {
    /// Jeton Bearer utilisé pour les requêtes API.
    pub access_token: String,

    /// Durée de validité du jeton d'accès, en secondes.
    pub expires_in: u64,

    /// Jeton permettant d'obtenir un nouvel access token, si fourni.
    pub refresh_token: Option<String>,

    /// Permissions OAuth accordées.
    pub scope: String,

    /// Type du jeton retourné par Qonto.
    pub token_type: String,
}

/// Client dédié aux échanges avec le serveur OAuth de Qonto.
pub struct QontoOAuthClient {
    http: reqwest::Client,
    client_id: String,
    client_secret: String,
}

impl QontoOAuthClient {
    /// Crée un client OAuth à partir des identifiants de l'application.
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Result<Self, QontoError> {
        let client_id = client_id.into();
        let client_secret = client_secret.into();

        if client_id.trim().is_empty() || client_secret.trim().is_empty() {
            return Err(QontoError::Configuration(
                "Les identifiants de l'application OAuth sont vides".into(),
            ));
        }

        Ok(Self {
            http: reqwest::Client::new(),
            client_id,
            client_secret,
        })
    }

    /// Construit l'URL vers laquelle rediriger l'utilisateur pour son consentement.
    ///
    /// `state` doit être généré aléatoirement et vérifié au retour OAuth.
    pub fn authorization_url(
        &self,
        redirect_uri: &str,
        scopes: &[&str],
        state: &str,
    ) -> Result<String, QontoError> {
        if redirect_uri.trim().is_empty() || state.trim().is_empty() {
            return Err(QontoError::Configuration(
                "redirect_uri et state sont obligatoires".into(),
            ));
        }

        let mut url = Url::parse(AUTHORIZATION_URL)
            .map_err(|error| QontoError::Configuration(format!("URL OAuth invalide : {error}")))?;

        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("scope", &scopes.join(" "))
            .append_pair("state", state);

        Ok(url.into())
    }

    /// Échange le code d'autorisation reçu après le consentement.
    pub async fn exchange_code(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<OAuthTokens, QontoError> {
        if code.trim().is_empty() || redirect_uri.trim().is_empty() {
            return Err(QontoError::Configuration(
                "Le code OAuth et redirect_uri sont obligatoires".into(),
            ));
        }

        let response = self
            .http
            .post(TOKEN_URL)
            .form(&[
                ("grant_type", "authorization_code"),
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("code", code),
                ("redirect_uri", redirect_uri),
            ])
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<OAuthTokens>().await?)
    }

    /// Renouvelle le jeton d'accès avec le dernier refresh token obtenu.
    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<OAuthTokens, QontoError> {
        if refresh_token.trim().is_empty() {
            return Err(QontoError::Configuration(
                "Le refresh token est vide".into(),
            ));
        }

        let response = self
            .http
            .post(TOKEN_URL)
            .form(&[
                ("grant_type", "refresh_token"),
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("refresh_token", refresh_token),
            ])
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<OAuthTokens>().await?)
    }
}
