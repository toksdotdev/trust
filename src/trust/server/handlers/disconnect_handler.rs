use crate::trust::server::ChatServer;
use actix::{Context, Handler};

/// Disconnect a client message.
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user_id: String,
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("User with id: [{}] disconnected", &msg.user_id);
        self.evict_user_from_server(&msg.user_id);
        self.broadcast_to_room_of_user(
            &msg.user_id,
            &format!("{} has left<NL>", &msg.user_id),
            &[],
        );
    }
}
