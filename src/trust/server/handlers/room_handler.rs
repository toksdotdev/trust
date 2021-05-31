use crate::trust::server::{ChatServer, ChatServerError};
use actix::{Context, Handler};

#[derive(actix::Message)]
#[rtype(result = "Result<String, ChatServerError>")]
pub enum ChatRoomCommand {
    Join {
        user_id: String,
        username: String,
        room_name: String,
        raw: String,
    },
    BroadcastMessage {
        user_id: String,
        content: String,
    },
}

/// Handler for Chat Server Command message.
impl<'e> Handler<ChatRoomCommand> for ChatServer {
    type Result = Result<String, ChatServerError>;

    fn handle(&mut self, command: ChatRoomCommand, _: &mut Context<Self>) -> Self::Result {
        match command {
            ChatRoomCommand::Join {
                user_id,
                raw,
                username,
                room_name,
            } => match self.get_user_room(&user_id) {
                Some(room_name) => {
                    if let Some(username) = self.get_username(&user_id) {
                        let message = format_user_message(&username, &raw);
                        self.broadcast_to_room(&room_name, &&message, &[]);
                    }
                }

                None => {
                    self.add_user_to_room(&room_name, &user_id, &username)?;
                    self.broadcast_to_room(&room_name, &format_user_has_joined(&username), &[]);
                }
            },

            ChatRoomCommand::BroadcastMessage { content, user_id } => {
                match self.get_username(&user_id) {
                    None => self.message_user(&user_id, &error_message()),
                    Some(username) => {
                        let message = format_user_message(&username, &content);
                        self.broadcast_to_room_of_user(&user_id, &&message, &[])
                    }
                }
            }
        }

        Ok("".to_string())
    }
}

/// Format message from user.
fn format_user_message(username: &str, message: &str) -> String {
    format!("{}: {}<NL>", username, message)
}

/// Format new user has joined message.
fn format_user_has_joined(username: &str) -> String {
    format!("{} has joined<NL>", &username)
}

/// Format new user has joined message.
fn error_message() -> String {
    "ERROR<NL>".to_string()
}
