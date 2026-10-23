use sdk_qonto_v2::{QontoClient, QontoConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Charge les variables d'environnement depuis .env.
    dotenvy::dotenv().ok();

    // Initialise le client Qonto.
    let config = QontoConfig::from_env()?;
    let client = QontoClient::new(config)?;

    // Récupère les comptes bancaires.
    let accounts = client.list_accounts().await?;

    let Some(account) = accounts.first() else {
        println!("Aucun compte Qonto trouvé.");
        return Ok(());
    };

    println!(
        "Compte sélectionné : {}",
        account.name.as_deref().unwrap_or("Nom non renseigné")
    );

    // Récupère la première page, avec 10 transactions maximum.
    let result = client.list_transactions_page(&account.id, 1, 10).await?;

    println!(
        "\nPage {} sur {}",
        result.pagination.current_page, result.pagination.total_pages
    );

    println!(
        "Nombre total de transactions : {}",
        result.pagination.total_count
    );

    // Affiche les informations utiles de chaque transaction.
    for (index, transaction) in result.transactions.iter().enumerate() {
        let date = transaction
            .settled_at
            .as_deref()
            .or(transaction.emitted_at.as_deref())
            .unwrap_or("Date inconnue");

        let label = transaction
            .label
            .as_deref()
            .unwrap_or("Libellé non renseigné");

        let currency = transaction.currency.as_deref().unwrap_or("Devise inconnue");

        println!("\nTransaction {}", index + 1);
        println!("  Date    : {}", date);
        println!("  Libellé : {}", label);

        match transaction.amount_cents {
            Some(cents) => {
                let sign = match transaction.side.as_deref() {
                    Some("debit") => "-",
                    Some("credit") => "+",
                    _ => "",
                };

                let amount = cents.unsigned_abs();

                println!(
                    "  Montant : {}{}.{:02} {}",
                    sign,
                    amount / 100,
                    amount % 100,
                    currency
                );
            }
            None => {
                println!("  Montant : Non renseigné");
            }
        }
    }

    Ok(())
}
