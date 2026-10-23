use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::Html,
    routing::get,
};
use sdk_qonto_v2::QontoOAuthClient;
use serde::Deserialize;
use std::{
    env,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    expected_state: String,
    redirect_uri: String,
    oauth: Arc<QontoOAuthClient>,
    tokens_received: Arc<Mutex<bool>>,
}

#[derive(Deserialize)]
struct CallbackParams {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

async fn callback(
    State(state): State<AppState>,
    Query(params): Query<CallbackParams>,
) -> (StatusCode, Html<String>) {
    if params.state.as_deref() != Some(state.expected_state.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Html("Paramètre state invalide.".into()),
        );
    }

    if let Some(error) = params.error {
        eprintln!("Qonto a refusé l'autorisation : {error}");
        return (
            StatusCode::BAD_REQUEST,
            Html("Autorisation refusée. Consulte le terminal.".into()),
        );
    }

    let Some(code) = params.code else {
        return (StatusCode::BAD_REQUEST, Html("Code OAuth absent.".into()));
    };

    if code.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Html("Code OAuth vide.".into()));
    }

    // Échange le code reçu contre les jetons OAuth.
    let tokens = match state.oauth.exchange_code(&code, &state.redirect_uri).await {
        Ok(tokens) => tokens,
        Err(_) => {
            eprintln!(
                "Échec de l'échange OAuth. Vérifie la configuration \
                 de l'application et le redirect_uri."
            );

            return (
                StatusCode::BAD_GATEWAY,
                Html("Impossible d'obtenir les jetons OAuth.".into()),
            );
        }
    };

    // Ne jamais afficher les jetons dans les logs.
    let expires_in = tokens.expires_in;
    let has_refresh_token = tokens.refresh_token.is_some();

    // Pour ce premier exemple, on ne conserve que le statut du succès.
    // L'étape suivante permettra de conserver les jetons pour les requêtes API.
    drop(tokens);

    match state.tokens_received.lock() {
        Ok(mut received) => *received = true,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Erreur interne.".into()),
            );
        }
    }

    println!("Échange OAuth réussi.");
    println!("Durée de validité du jeton : {expires_in} secondes.");
    println!("Refresh token reçu : {has_refresh_token}.");

    (
        StatusCode::OK,
        Html("Connexion OAuth réussie. Tu peux fermer cette page.".into()),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client_id = env::var("QONTO_OAUTH_CLIENT_ID")?;
    let client_secret = env::var("QONTO_OAUTH_CLIENT_SECRET")?;
    let redirect_uri = env::var("QONTO_OAUTH_REDIRECT_URI")?;

    let oauth = Arc::new(QontoOAuthClient::new(client_id, client_secret)?);
    let state_value = Uuid::new_v4().to_string();

    let authorization_url = oauth.authorization_url(
        &redirect_uri,
        &["organization.read", "webhook", "offline_access"],
        &state_value,
    )?;

    let app_state = AppState {
        expected_state: state_value,
        redirect_uri,
        oauth,
        tokens_received: Arc::new(Mutex::new(false)),
    };

    let app = Router::new()
        .route("/callback", get(callback))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    println!("Serveur prêt sur http://localhost:8080/callback");
    println!("\nOuvre cette URL dans ton navigateur :\n");
    println!("{authorization_url}\n");

    axum::serve(listener, app).await?;

    Ok(())
}
