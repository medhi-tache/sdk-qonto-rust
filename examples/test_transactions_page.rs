use sdk_qonto_v2::{QontoClient, QontoConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = QontoConfig::from_env()?;
    let client = QontoClient::new(config)?;

    // Récupère les comptes sans afficher leurs identifiants.
    let accounts = client.list_accounts().await?;

    let Some(account) = accounts.first() else {
        println!("Aucun compte trouvé.");
        return Ok(());
    };

    // Teste la première page avec 10 transactions maximum.
    let result = client.list_transactions_page(&account.id, 1, 10).await?;

    println!("Requête réussie.");
    println!(
        "Page : {} / {}",
        result.pagination.current_page, result.pagination.total_pages
    );
    println!("Taille de page : {}", result.pagination.per_page);
    println!(
        "Nombre total de transactions : {}",
        result.pagination.total_count
    );
    println!(
        "Transactions reçues sur cette page : {}",
        result.transactions.len()
    );
    println!(
        "Page suivante disponible : {}",
        result.pagination.next_page.is_some()
    );

    // Affiche uniquement les noms des champs du modèle,
    // jamais les valeurs des transactions.
    if let Some(transaction) = result.transactions.first() {
        println!("Champs du modèle Transaction disponibles :");
        println!("  id : {}", transaction.id.is_some());
        println!(
            "  transaction_id : {}",
            transaction.transaction_id.is_some()
        );
        println!("  label : {}", transaction.label.is_some());
        println!("  amount_cents : {}", transaction.amount_cents.is_some());
        println!("  currency : {}", transaction.currency.is_some());
        println!("  side : {}", transaction.side.is_some());
        println!("  settled_at : {}", transaction.settled_at.is_some());
        println!("  emitted_at : {}", transaction.emitted_at.is_some());
        println!("  status : {}", transaction.status.is_some());
    } else {
        println!("Aucune transaction sur cette page.");
    }

    Ok(())
}
