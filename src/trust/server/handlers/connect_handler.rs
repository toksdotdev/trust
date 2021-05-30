use super::Message;
use crate::trust::server::ChatServer;
use crate::trust::server::ChatServerError;
use actix::Recipient;
use actix::{Context, Handler};

/// New Client has connected.
#[derive(actix::Message)]
#[rtype(result = "Result<String, ChatServerError>")]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Handler for Connect message.
impl Handler<Connect> for ChatServer {
    type Result = Result<String, ChatServerError>;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone just connected!!!");
        self.handle_new_connection(msg.addr)
    }
}
