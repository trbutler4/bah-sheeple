mod config;
mod handlers;
mod services;

use actix_web::{web, App, HttpServer};
use oauth2::basic::BasicClient;
use oauth2::AccessToken;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

use crate::config::Config;
use crate::handlers::{auth, tweets};

pub struct AppState {
    client: Mutex<BasicClient>,
    verifier: Arc<Mutex<Option<oauth2::PkceCodeVerifier>>>,
    access_token: Arc<Mutex<Option<AccessToken>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let client = config.create_oauth_client();

    let shared_verifier = Arc::new(Mutex::new(None));
    let shared_verifier_clone = shared_verifier.clone();

    let shared_token = Arc::new(Mutex::new(None));
    let shared_token_clone = shared_token.clone();

    // app data that will use in both the server and background task
    let app_data = web::Data::new(AppState {
        client: Mutex::new(client.clone()),
        verifier: shared_verifier_clone.clone(),
        access_token: shared_token_clone.clone(),
    });

    let app_data_clone = app_data.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            match handlers::tweets::generate_and_post_tweet(app_data_clone.clone()).await {
                Ok(result) => println!("Scheduled tweet result: {}", result),
                Err(e) => println!("Scheduled tweet error: {}", e),
            }
        }
    });

    println!("Starting server on: http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                client: Mutex::new(client.clone()),
                verifier: shared_verifier_clone.clone(),
                access_token: shared_token_clone.clone(),
            }))
            .service(auth::start)
            .service(auth::oauth_callback)
            .service(tweets::post_tweet)
            .service(tweets::bah)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
