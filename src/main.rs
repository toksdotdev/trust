use actix_web::{App, HttpServer};
use loaders::trust_chat_handler;

mod loaders;
mod trust;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/trust/chat", trust_chat_handler()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
