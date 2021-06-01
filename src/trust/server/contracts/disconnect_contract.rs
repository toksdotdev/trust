use crate::trust::server::ChatServer;
use actix::{Context, Handler};

/// Disconnect a client message.
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct DisconnectContract {
    pub user_id: String,
}

/// Handler for Disconnect message.
impl Handler<DisconnectContract> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: DisconnectContract, _: &mut Context<Self>) {
        println!("User with id: [{}] disconnected", &msg.user_id);

        if let Some(username) = self.get_username(&msg.user_id) {
            self.evict_user_from_server(&msg.user_id);
            self.broadcast_to_room_of_user(&msg.user_id, &format_user_has_left(&&username), &[]);
        }
    }
}

/// Format new user has joined message.
fn format_user_has_left(username: &str) -> String {
    format!("{} has left<NL>", &username)
}
