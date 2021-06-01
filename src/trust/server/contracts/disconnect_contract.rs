use crate::{
    log,
    trust::{response::user_left_message, server::TrustServer},
};
use actix::{Context, Handler};

/// Disconnect a client message.
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct DisconnectContract {
    pub user_id: String,
}

/// Handler for Disconnect message.
impl Handler<DisconnectContract> for TrustServer {
    type Result = ();

    fn handle(&mut self, msg: DisconnectContract, _: &mut Context<Self>) {
        log!("User with id: [{}] disconnected", &msg.user_id);

        if let Some(username) = self.get_username(&msg.user_id) {
            self.evict_user_from_server(&msg.user_id);
            self.broadcast_to_room_of_user(&msg.user_id, &user_left_message(&&username), &[]);
        }
    }
}
