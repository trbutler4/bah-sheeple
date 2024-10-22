use actix_web::{get, web, App, HttpServer, Responder};
use anyhow;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,
};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::sync::Mutex;

struct AppState {
    client: Mutex<BasicClient>,
}

#[get("/")]
async fn start(data: web::Data<AppState>) -> impl Responder {
    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

    // Generate the full authorization URL.
    let client = &data.client.lock().unwrap();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("tweet.read".to_string()))
        .add_scope(Scope::new("tweet.write".to_string()))
        .add_scope(Scope::new("users.read".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);
    format!("Browse to: {}", auth_url)
}

#[get("/oauth/callback")]
async fn oauth_callback() -> impl Responder {
    println!("Callback triggered");
    format!("callback triggered")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_base_url = String::from("https://proud-maggot-initially.ngrok-free.app");
    let twitter_api_base_url = String::from("https://api.x.com");
    let auth_url = format!("{}/oauth/request_token", twitter_api_base_url);
    let redirect_url = format!("{}/oauth/callback", app_base_url);
    let token_url = format!("{}/oauth/access_token", twitter_api_base_url);

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new("client_id".to_string()),
        Some(ClientSecret::new("client_secret".to_string())),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                client: Mutex::new(client.clone()),
            }))
            .service(start)
            .service(oauth_callback)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
