use actix::Actor;
use actix_web::{App, HttpServer};
use loaders::setup_tcp;
use trust::server::ChatServer;

mod loaders;
mod trust;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = ChatServer::default().start();
    let srv = server.clone();
    let handle = setup_tcp("127.0.0.1:1238".parse().unwrap(), srv);

    HttpServer::new(|| App::new())
        .bind("127.0.0.1:8086")?
        .run()
        .await
        .and_then(|_| Ok(handle.abort()))
}
