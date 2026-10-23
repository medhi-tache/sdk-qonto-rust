use serde::Deserialize;

/// Représente une transaction Qonto.
#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    /// Identifiant technique de la transaction.
    pub id: Option<String>,

    /// Référence de transaction fournie par Qonto.
    pub transaction_id: Option<String>,

    /// Libellé de la transaction ou nom de la contrepartie.
    pub label: Option<String>,

    /// Montant en centimes, sous forme d'entier.
    pub amount_cents: Option<i64>,

    /// Devise de la transaction, par exemple EUR.
    pub currency: Option<String>,

    /// Sens du mouvement : "debit" ou "credit".
    pub side: Option<String>,

    /// Date de règlement, absente notamment pour certaines opérations en attente.
    pub settled_at: Option<String>,

    /// Date d'émission de la transaction.
    pub emitted_at: Option<String>,

    /// Statut de la transaction.
    pub status: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_transaction_from_json() {
        let json = serde_json::json!({
            "id": "transaction-test-123",
            "transaction_id": "reference-test",
            "label": "Transaction de test",
            "amount_cents": 12345,
            "currency": "EUR",
            "side": "debit",
            "settled_at": "2026-10-20T10:30:00Z",
            "emitted_at": "2026-10-20T09:30:00Z",
            "status": "completed"
        });

        let transaction: Transaction =
            serde_json::from_value(json).expect("Transaction devrait être désérialisée");

        assert_eq!(transaction.id.as_deref(), Some("transaction-test-123"));
        assert_eq!(transaction.label.as_deref(), Some("Transaction de test"));
        assert_eq!(transaction.amount_cents, Some(12345));
        assert_eq!(transaction.currency.as_deref(), Some("EUR"));
        assert_eq!(transaction.side.as_deref(), Some("debit"));
        assert_eq!(
            transaction.settled_at.as_deref(),
            Some("2026-10-20T10:30:00Z")
        );
    }

    #[test]
    fn deserialize_transaction_with_optional_fields_missing() {
        let json = serde_json::json!({
            "amount_cents": 500
        });

        let transaction: Transaction =
            serde_json::from_value(json).expect("Les champs optionnels peuvent manquer");

        assert_eq!(transaction.amount_cents, Some(500));
        assert_eq!(transaction.label, None);
        assert_eq!(transaction.currency, None);
    }

    #[test]
    fn amount_cents_remains_an_integer() {
        let json = serde_json::json!({
            "amount_cents": 12345
        });

        let transaction: Transaction =
            serde_json::from_value(json).expect("Le montant devrait être désérialisé");

        assert_eq!(transaction.amount_cents, Some(12_345_i64));
    }
}
