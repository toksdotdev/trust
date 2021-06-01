use crate::trust::{
    response::{error_message, new_user_message, user_joined_message},
    server::{TrustServer, TrustServerError},
};
use actix::{Context, Handler};

#[derive(actix::Message)]
#[rtype(result = "Result<String, TrustServerError>")]
pub enum RoomContract {
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
impl<'e> Handler<RoomContract> for TrustServer {
    type Result = Result<String, TrustServerError>;

    fn handle(&mut self, command: RoomContract, _: &mut Context<Self>) -> Self::Result {
        match command {
            RoomContract::Join {
                user_id,
                raw,
                username,
                room_name,
            } => match self.get_user_room(&user_id) {
                Some(room_name) => {
                    if let Some(username) = self.get_username(&user_id) {
                        let message = new_user_message(&username, &raw);
                        self.broadcast_to_room(&room_name, &&message, &[]);
                    }
                }

                None => {
                    self.add_user_to_room(&room_name, &user_id, &username)?;
                    self.broadcast_to_room(&room_name, &user_joined_message(&username), &[]);
                }
            },

            RoomContract::BroadcastMessage { content, user_id } => {
                match self.get_username(&user_id) {
                    None => self.message_user(&user_id, &error_message()),
                    Some(username) => {
                        let message = new_user_message(&username, &content);
                        self.broadcast_to_room_of_user(&user_id, &&message, &[])
                    }
                }
            }
        }

        Ok("".to_string())
    }
}
