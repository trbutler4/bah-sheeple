use actix_web::{get, web, Responder};
use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, RequestTokenError, Scope, TokenResponse,
};
use serde::Deserialize;

use crate::services::openai::OpenAIService;
use crate::AppState;

use oauth2::reqwest::async_http_client;

#[derive(Deserialize)]
pub struct CallbackParams {
    pub state: String,
    pub code: String,
}

#[get("/")]
pub async fn start(data: web::Data<AppState>) -> impl Responder {
    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Store the verifier
    *data.verifier.lock().unwrap() = Some(pkce_verifier);

    // Generate the full authorization URL.
    let client = &data.client.lock().unwrap();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("tweet.read".to_string()))
        .add_scope(Scope::new("tweet.write".to_string()))
        .add_scope(Scope::new("users.read".to_string()))
        .add_scope(Scope::new("offline.access".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);
    format!("Browse to: {}", auth_url)
}

#[get("/oauth/callback")]
pub async fn oauth_callback(
    data: web::Data<AppState>,
    params: web::Query<CallbackParams>,
) -> impl Responder {
    // loading stored verifier
    let pkce_verifier = data
        .verifier
        .lock()
        .unwrap()
        .take()
        .expect("PKCE verifier not found");

    // Now you can trade it for an access token.
    let client = &data.client.lock().unwrap();
    let token_result = client
        .exchange_code(AuthorizationCode::new(params.code.clone()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;

    match &token_result {
        Ok(token) => {
            println!("Storing access token: {}", token.access_token().secret());
            *data.access_token.lock().unwrap() = Some(token.access_token().clone());
            return format!("Authentication successful! You can now use /tweet to post tweets.");
        }
        Err(e) => {
            match e {
                RequestTokenError::ServerResponse(err) => {
                    println!("Server error: {:?}", err);
                }
                RequestTokenError::Request(err) => {
                    println!("Request error: {:?}", err);
                }
                RequestTokenError::Parse(err, response) => {
                    println!("Parse error: {:?}, Response: {:?}", err, response);
                }
                RequestTokenError::Other(err) => {
                    println!("Other error: {:?}", err);
                }
            }
            return format!("Failed to get token");
        }
    }
}
