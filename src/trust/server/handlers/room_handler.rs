use crate::trust::server::{ChatServer, ChatServerError};
use actix::{Context, Handler};

pub struct JoinChatRoom {
    pub user_id: String,
    pub username: String,
    pub room_name: String,
}

#[derive(actix::Message)]
#[rtype(result = "Result<String, ChatServerError>")]
pub enum ChatServerCommand {
    JoinChatRoom(JoinChatRoom),
}

/// Handler for Chat Server Command message.
impl<'e> Handler<ChatServerCommand> for ChatServer {
    type Result = Result<String, ChatServerError>;

    fn handle(&mut self, command: ChatServerCommand, _: &mut Context<Self>) -> Self::Result {
        match command {
            ChatServerCommand::JoinChatRoom(ref payload) => {
                self.add_user_to_room(payload)?;
                self.broadcast_to_room(
                    &payload.room_name,
                    &format!("{} has joined<NL>", payload.username),
                    &[],
                );
            }
        }

        Ok("".to_string())
    }
}
