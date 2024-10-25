use actix_web::{get, web, App, HttpServer, Responder};
use anyhow;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RequestTokenError, Scope, StandardErrorResponse,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use rand;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

mod utils;

struct AppState {
    client: Mutex<BasicClient>,
    verifier: Arc<Mutex<Option<PkceCodeVerifier>>>,
}

#[derive(Deserialize)]
struct CallbackParams {
    state: String,
    code: String,
}

#[get("/")]
async fn start(data: web::Data<AppState>) -> impl Responder {
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
async fn oauth_callback(
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
        Ok(r) => println!("got standard token response"),
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

    let message = String::from("bahhhhhhh");

    let twitter_api_url = "https://api.twitter.com/2/tweets";

    let tweet_data = serde_json::json!({
        "text": message
    });

    let client = reqwest::Client::new();
    let response = client
        .post(twitter_api_url)
        .header(
            "Authorization",
            format!("Bearer {}", token_result.unwrap().access_token().secret()),
        )
        .header("Content-Type", "application/json")
        .body(tweet_data.to_string())
        .send()
        .await
        .unwrap();

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if status.is_success() {
        println!("Tweet posted successfully!");
        println!("Response: {}", body);
        return format!("Posted tweet: {}", message);
    } else {
        println!("Failed to post tweet - Status: {}", status);
        println!("Error response: {}", body);
        return format!("Failed to post tweet: {} - {}", status, body);
    }
}

#[get("/bah")]
async fn bah() -> impl Responder {
    let mut result = String::from("ba");
    let num_h = rand::random::<u8>() % 15 + 4;
    result.extend(std::iter::repeat('h').take(num_h as usize));
    format!("{}", result)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_base_url = String::from("https://proud-maggot-initially.ngrok-free.app");
    let auth_url = format!("https://twitter.com/i/oauth2/authorize");
    let redirect_url = format!("{}/oauth/callback", app_base_url);
    let token_url = format!("https://api.twitter.com/2/oauth2/token");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Read client ID from .env file
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set in .env file");
    let client_secret =
        std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set in .env file");

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // shared verifier state
    let shared_verifier = Arc::new(Mutex::new(None));
    let shared_verifier_clone = shared_verifier.clone();

    println!("Starting bah on: http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                client: Mutex::new(client.clone()),
                verifier: shared_verifier_clone.clone(),
            }))
            .service(start)
            .service(oauth_callback)
            .service(bah)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
