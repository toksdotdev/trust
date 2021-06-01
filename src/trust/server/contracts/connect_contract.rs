use super::PlainTextMessage;
use crate::log;
use crate::trust::server::TrustServer;
use crate::trust::server::TrustServerError;
use actix::Recipient;
use actix::{Context, Handler};

/// Connect a client message.
#[derive(actix::Message)]
#[rtype(result = "Result<String, TrustServerError>")]
pub struct ConnectContract {
    pub addr: Recipient<PlainTextMessage>,
}

/// Handler for Connect message.
impl Handler<ConnectContract> for TrustServer {
    type Result = Result<String, TrustServerError>;

    fn handle(&mut self, msg: ConnectContract, _: &mut Context<Self>) -> Self::Result {
        log!("Someone just connected!!!");
        self.handle_new_connection(msg.addr)
    }
}
