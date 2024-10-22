use actix_web::{get, web, App, HttpServer, Responder};
use oauth2::PkceCodeVerifier;
use std::sync::Mutex;

mod oauth_client;

static PKCE_CHALLENGE: Mutex<Option<oauth2::PkceCodeChallenge>> = Mutex::new(None);
static PKCE_VERIFIER: Mutex<Option<PkceCodeVerifier>> = Mutex::new(None);

#[get("/bah")]
async fn bah() -> impl Responder {
    format!("bahhhhhhh")
}

#[get("/connect")]
async fn connect() -> impl Responder {
    println!("Attemptin to create client");
    let client = oauth_client::create_client().unwrap();

    let (pkce_challenge, pkce_verifier) = oauth_client::genetate_challenge();

    *PKCE_CHALLENGE.lock().unwrap() = Some(pkce_challenge.clone());
    *PKCE_VERIFIER.lock().unwrap() = Some(pkce_verifier);

    let (auth_url, csrf_token) = oauth_client::generate_authorization_url(client, pkce_challenge);

    println!("Connect to {}", auth_url);

    format!("Connet to: {}", auth_url)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(bah).service(connect))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
