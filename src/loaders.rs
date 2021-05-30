use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse, Route};
use actix_web_actors::ws;
use web::{Data, Payload};

use crate::trust::{session::ChatSession, server::ChatServer};

/// Trust
pub(crate) fn trust_chat_handler() -> Route {
    web::get().to(startup_trust_chat)
}

/// Startup the Trust chat server.
async fn startup_trust_chat(
    req: HttpRequest,
    stream: Payload,
    chat_server: Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        ChatSession::new(chat_server.get_ref().clone()),
        &req,
        stream,
    )
}
