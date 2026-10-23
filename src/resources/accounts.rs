use reqwest::Method;
use serde::Deserialize;

use crate::{Account, QontoClient, QontoError};

#[derive(Debug, Deserialize)]
struct OrganizationResponse {
    organization: OrganizationData,
}

#[derive(Debug, Deserialize)]
struct OrganizationData {
    bank_accounts: Vec<Account>,
}

impl QontoClient {
    /// Récupère les comptes de l'organisation authentifiée.
    pub async fn list_accounts(&self) -> Result<Vec<Account>, QontoError> {
        let response: OrganizationResponse =
            self.request(Method::GET, "organization", None).await?;

        Ok(response.organization.bank_accounts)
    }
}
