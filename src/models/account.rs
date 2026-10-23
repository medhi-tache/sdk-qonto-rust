use serde::Deserialize;

/// Représente un compte bancaire de l'organisation Qonto.
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    /// Identifiant unique du compte.
    pub id: String,

    /// Nom du compte.
    pub name: Option<String>,

    /// Identifiant lisible du compte.
    pub slug: Option<String>,

    /// Numéro de compte, si disponible.
    pub account_number: Option<String>,

    /// IBAN du compte.
    pub iban: Option<String>,

    /// Code BIC/SWIFT.
    pub bic: Option<String>,

    /// Devise du compte, par exemple EUR.
    pub currency: Option<String>,

    /// Solde du compte, exprimé dans l'unité monétaire.
    pub balance: Option<f64>,

    /// Solde du compte en centimes, si fourni.
    pub balance_cents: Option<i64>,

    /// Solde autorisé, exprimé dans l'unité monétaire.
    pub authorized_balance: Option<f64>,

    /// Solde autorisé en centimes, si fourni.
    pub authorized_balance_cents: Option<i64>,

    /// Indique si ce compte est le compte principal.
    pub main: Option<bool>,

    /// Indique si le compte est un compte externe.
    pub is_external_account: Option<bool>,

    /// Statut actuel du compte.
    pub status: Option<String>,

    /// Date de dernière mise à jour, telle que renvoyée par l'API.
    pub updated_at: Option<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_account_from_json() {
        let json = serde_json::json!({
            "id": "account-test-123",
            "name": "Compte de test",
            "currency": "EUR",
            "status": "active",
            "main": true
        });

        let account: Account =
            serde_json::from_value(json).expect("Account devrait être désérialisé");

        assert_eq!(account.id, "account-test-123");
        assert_eq!(account.name.as_deref(), Some("Compte de test"));
        assert_eq!(account.currency.as_deref(), Some("EUR"));
        assert_eq!(account.status.as_deref(), Some("active"));
        assert_eq!(account.main, Some(true));
    }

    #[test]
    fn deserialize_account_with_optional_fields_missing() {
        let json = serde_json::json!({
            "id": "account-minimal"
        });

        let account: Account =
            serde_json::from_value(json).expect("Les champs optionnels peuvent manquer");

        assert_eq!(account.id, "account-minimal");
        assert_eq!(account.name, None);
        assert_eq!(account.currency, None);
        assert_eq!(account.status, None);
    }
}
