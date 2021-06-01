use actix::Actor;
use loaders::start_tcp_listener;
use structopt::StructOpt;
use trust::server::TrustServer;
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
    let server = TrustServer::default().start();
    let address = format!("127.0.0.1:{}", args.port).parse().unwrap();
    log!("Starting application on {:?}", address);

    let _ = start_tcp_listener(address, server.clone()).await;
    Ok(())
}
