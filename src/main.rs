use actix_web::{get, web, App, HttpServer, Responder};

mod oauth_client;

#[get("/")]
async fn bah() -> impl Responder {
    format!("bahhhhhhh")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
