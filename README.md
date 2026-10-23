# SDK Qonto pour Rust

Un SDK Rust non officiel pour interagir avec l’API Qonto. Il permet de configurer un client HTTP et de récupérer les comptes bancaires ainsi que les transactions d’une organisation.

> **Statut :** en développement. Les fonctionnalités et les modèles de données peuvent évoluer.

## Fonctionnalités

- Configuration du client à partir de variables d’environnement.
- Authentification aux requêtes API.
- Récupération des comptes bancaires.
- Récupération des transactions d’un compte.
- Pagination des transactions.
- Gestion des erreurs HTTP, API et JSON.

## Prérequis

- Rust et Cargo installés.
- Des identifiants d’accès à l’API Qonto.
- Un accès autorisé à l’API Qonto.

## Installation

Si tu utilises ce SDK dans un autre projet Rust, ajoute-le comme dépendance depuis son dépôt Git :

```toml
[dependencies]
sdk-qonto-v2 = { git = "URL_DU_DEPOT_GIT" }
```

Remplace `URL_DU_DEPOT_GIT` par l’URL réelle du dépôt une fois le projet publié.

Pour travailler directement sur le projet :

```bash
git clone URL_DU_DEPOT_GIT
cd sdk-qonto-v2
cargo build
```

## Configuration

Le SDK peut être configuré avec les variables d’environnement suivantes :

| Variable | Description |
|---|---|
| `QONTO_BASE_URL` | URL de base de l’API |
| `QONTO_LOGIN` | Identifiant d’authentification |
| `QONTO_SECRET` | Secret d’authentification |

Exemple de fichier `.env` local :

```dotenv
QONTO_BASE_URL=https://thirdparty.qonto.com/v2
QONTO_LOGIN=votre_identifiant
QONTO_SECRET=votre_secret
```


## Exemples d’utilisation

### Récupérer les comptes bancaires

```rust
use sdk_qonto_v2::{QontoClient, QontoConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = QontoConfig::from_env()?;
    let client = QontoClient::new(config)?;

    let accounts = client.list_accounts().await?;

    println!("Nombre de comptes : {}", accounts.len());

    for account in accounts {
        println!("Compte : {}", account.id);

        if let Some(name) = account.name {
            println!("Nom : {}", name);
        }
    }

    Ok(())
}
```

### Récupérer les transactions

```rust
use sdk_qonto_v2::{QontoClient, QontoConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = QontoConfig::from_env()?;
    let client = QontoClient::new(config)?;

    let accounts = client.list_accounts().await?;

    if let Some(account) = accounts.first() {
        let transactions = client.list_transactions(&account.id).await?;

        println!("Nombre de transactions : {}", transactions.len());

        for transaction in transactions {
            println!(
                "{} : {} centimes",
                transaction.label.as_deref().unwrap_or("Sans libellé"),
                transaction.amount_cents.unwrap_or(0)
            );
        }
    }

    Ok(())
}
```

Ces exemples supposent que les noms des méthodes et les types correspondent à la version actuelle du SDK.

## Pagination

Le SDK propose deux méthodes :

- `list_transactions_page(...)` pour récupérer une page précise et ses métadonnées.
- `list_transactions(...)` pour récupérer les transactions sur plusieurs pages.

La seconde méthode rassemble les résultats des pages successives jusqu’à ce qu’il n’y ait plus de page suivante.

## Gestion des erreurs

Les méthodes du client renvoient un `Result`, ce qui permet de traiter les erreurs avec l’opérateur `?` ou avec un bloc `match`.

Le type `QontoError` distingue notamment :

- les erreurs HTTP ;
- les erreurs de configuration ;
- les erreurs retournées par l’API ;
- les erreurs de désérialisation JSON.

## Développement

Pour compiler le projet :

```bash
cargo build
```

Pour lancer les tests :

```bash
cargo test
```

Pour vérifier le formatage :

```bash
cargo fmt --check
```

Pour analyser le code :

```bash
cargo clippy
```

## Structure du projet

```text
sdk-qonto-v2/
├── Cargo.toml
├── README.md
├── examples/
│   ├── list_accounts.rs
│   └── list_transactions.rs
└── src/
    ├── lib.rs
    ├── client.rs
    ├── config.rs
    ├── error.rs
    ├── models/
    │   ├── mod.rs
    │   ├── account.rs
    │   └── transaction.rs
    └── resources/
        ├── mod.rs
        ├── accounts.rs
        └── transactions.rs
```

## Licence

Ce projet est distribué sous la licence MIT, sous réserve de la présence du fichier `LICENSE` correspondant à la racine du dépôt.
