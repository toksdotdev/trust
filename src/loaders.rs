use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse, Route};
use actix_web_actors::ws;
use web::{Data, Payload};

use crate::trust::{network::TrustNetwork, server::ChatServer};

pub(crate) fn trust_chat_handler() -> Route {
    web::get().to(setup_trust_chat)
}

async fn setup_trust_chat(
    req: HttpRequest,
    stream: Payload,
    chat_server: Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        TrustNetwork::new(chat_server.get_ref().clone()),
        &req,
        stream,
    )
}
