use reqwest::Method;
use serde::Deserialize;

use crate::{QontoClient, QontoError, Transaction};

#[derive(Debug, Deserialize)]
struct TransactionsResponse {
    transactions: Vec<Transaction>,
    meta: PaginationMeta,
}

#[derive(Debug, Deserialize)]
pub struct PaginationMeta {
    pub current_page: u32,
    pub next_page: Option<u32>,
    pub prev_page: Option<u32>,
    pub total_pages: u32,
    pub total_count: u32,
    pub per_page: u32,
}

#[derive(Debug)]
pub struct TransactionsPage {
    pub transactions: Vec<Transaction>,
    pub pagination: PaginationMeta,
}

impl QontoClient {
    /// Récupère une page de transactions, des plus récentes aux plus anciennes.
    pub async fn list_transactions_page(
        &self,
        bank_account_id: &str,
        page: u32,
        per_page: u32,
    ) -> Result<TransactionsPage, QontoError> {
        if bank_account_id.is_empty()
            || !bank_account_id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Err(QontoError::Configuration(
                "Identifiant de compte bancaire invalide".to_string(),
            ));
        }

        if page == 0 || !(1..=100).contains(&per_page) {
            return Err(QontoError::Configuration(
                "page doit être supérieur à 0 et per_page compris entre 1 et 100".to_string(),
            ));
        }

        let path = format!(
            "transactions?bank_account_id={}&page={}&per_page={}&sort_by=settled_at:desc",
            bank_account_id, page, per_page
        );

        let response: TransactionsResponse = self.request(Method::GET, &path, None).await?;

        Ok(TransactionsPage {
            transactions: response.transactions,
            pagination: response.meta,
        })
    }

    /// Récupère toutes les pages de transactions du compte.
    pub async fn list_transactions(
        &self,
        bank_account_id: &str,
    ) -> Result<Vec<Transaction>, QontoError> {
        let mut all_transactions = Vec::new();
        let mut next_page = Some(1_u32);
        let per_page = 100_u32;

        while let Some(page_number) = next_page {
            let result = self
                .list_transactions_page(bank_account_id, page_number, per_page)
                .await?;

            // Évite une boucle infinie si la réponse de l'API est incohérente.
            let following_page = result.pagination.next_page;

            all_transactions.extend(result.transactions);

            if following_page.is_some_and(|next| next <= page_number) {
                return Err(QontoError::Configuration(
                    "La pagination Qonto a renvoyé un numéro de page invalide".to_string(),
                ));
            }

            next_page = following_page;
        }

        Ok(all_transactions)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_pagination_with_next_page() {
        let json = serde_json::json!({
            "current_page": 2,
            "next_page": 3,
            "prev_page": 1,
            "total_pages": 5,
            "total_count": 95,
            "per_page": 20
        });

        let pagination: PaginationMeta =
            serde_json::from_value(json).expect("Pagination valide attendue");

        assert_eq!(pagination.current_page, 2);
        assert_eq!(pagination.next_page, Some(3));
        assert_eq!(pagination.prev_page, Some(1));
        assert_eq!(pagination.total_pages, 5);
        assert_eq!(pagination.total_count, 95);
        assert_eq!(pagination.per_page, 20);
    }

    #[test]
    fn deserialize_pagination_on_last_page() {
        let json = serde_json::json!({
            "current_page": 5,
            "next_page": null,
            "prev_page": 4,
            "total_pages": 5,
            "total_count": 95,
            "per_page": 20
        });

        let pagination: PaginationMeta =
            serde_json::from_value(json).expect("Pagination finale attendue");

        assert_eq!(pagination.current_page, 5);
        assert_eq!(pagination.next_page, None);
        assert_eq!(pagination.prev_page, Some(4));
    }
}
