use actix::Actor;
use loaders::start_tcp_listener;
use structopt::StructOpt;
use trust::server::ChatServer;
mod loaders;
mod trust;

/// CLI Args
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct CliArgs {
    #[structopt(short, long, default_value = "1234")]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = CliArgs::from_args();
    let server = ChatServer::default().start();
    let address = format!("0.0.0.0:{}", args.port).parse().unwrap();
    println!("Starting application on {:?}", address);

    let _ = start_tcp_listener(address, server.clone()).await;
    Ok(())
}
