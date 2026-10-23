use sdk_qonto_v2::{QontoClient, QontoConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Charge les variables du fichier .env, sans afficher leurs valeurs.
    dotenvy::dotenv().ok();

    // Lit la configuration et crée le client Qonto.
    let config = QontoConfig::from_env()?;
    let client = QontoClient::new(config)?;

    // Récupère les comptes de l'organisation.
    let accounts = client.list_accounts().await?;

    println!("Nombre de comptes : {}", accounts.len());

    // Affiche uniquement les informations autorisées.
    for (index, account) in accounts.iter().enumerate() {
        println!("\nCompte {} :", index + 1);
        println!(
            "  Nom      : {}",
            account.name.as_deref().unwrap_or("Non renseigné")
        );
        println!(
            "  Devise   : {}",
            account.currency.as_deref().unwrap_or("Non renseignée")
        );
        println!(
            "  Statut   : {}",
            account.status.as_deref().unwrap_or("Non renseigné")
        );
    }

    Ok(())
}
