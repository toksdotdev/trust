use crate::trust::server::ChatServer;
use actix::{Context, Handler};

/// Client has disconnected
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
        self.remove_user_from_all_rooms(&msg.user_id);
        self.broadcast_to_all_rooms(&format!("{} has left<NL>", &msg.user_id), &[]);
    }
}
