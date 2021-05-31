use crate::trust::codec::TrustTcpChatCodec;
use crate::trust::server::ChatServer;
use crate::trust::user::User;
use actix::io::FramedWrite;
use actix::Actor;
use actix::Addr;
use actix::StreamHandler;
use std::net::SocketAddr;
use tokio::io::split;
use tokio::net::TcpListener;
use tokio_util::codec::FramedRead;

/// Setup TCP listener for Trust Chat Server on a socket address specified.
pub async fn start_tcp_listener(addr: SocketAddr, server: Addr<ChatServer>) {
    let server = server.clone();
    let listener = TcpListener::bind(addr).await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let server = server.clone();

        User::create(|ctx| {
            let (r, w) = split(stream);
            User::add_stream(FramedRead::new(r, TrustTcpChatCodec), ctx);
            User::new(server, FramedWrite::new(w, TrustTcpChatCodec, ctx))
        });
    }
}
