mod config;
mod handlers;
mod services;

use actix_web::{web, App, HttpServer};
use oauth2::basic::BasicClient;
use std::sync::{Arc, Mutex};

use crate::config::Config;
use crate::handlers::{auth, tweets};

pub struct AppState {
    client: Mutex<BasicClient>,
    verifier: Arc<Mutex<Option<oauth2::PkceCodeVerifier>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let client = config.create_oauth_client();

    let shared_verifier = Arc::new(Mutex::new(None));
    let shared_verifier_clone = shared_verifier.clone();

    println!("Starting server on: http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                client: Mutex::new(client.clone()),
                verifier: shared_verifier_clone.clone(),
            }))
            .service(auth::start)
            .service(auth::oauth_callback)
            .service(tweets::bah)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
