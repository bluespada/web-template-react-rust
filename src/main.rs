use actix_web::{web, App, HttpServer };
use tokio;

mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        // initialize frontend website
        .default_service(
            web::route().to(server::frontend)
        )
        // define your backend api here.
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}