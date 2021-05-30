use crate::trust::server::utils::{valid_chatroom_name, valid_username};
use std::str::FromStr;

pub enum ChatSessionCommand {
    JoinChatRoom { username: String, room_name: String },
}

impl FromStr for ChatSessionCommand {
    type Err = String;

    fn from_str(message: &str) -> Result<Self, Self::Err> {
        let message = message.replace("\n", "");
        let mut fragments = message.split_ascii_whitespace();

        let command = fragments.next();
        if command.is_none() {
            return Err("Command cannot be empty".to_string());
        }

        match command.unwrap().to_ascii_lowercase().as_str() {
            "join" => {
                let username = fragments.next();
                let room_name = fragments.next();
                if !valid_username(username) {
                    return Err("Invalid username".to_string());
                }

                if !valid_chatroom_name(room_name) {
                    return Err("Invalid room name".to_string());
                }

                Ok(Self::JoinChatRoom {
                    username: username.unwrap().to_string(),
                    room_name: room_name.unwrap().to_string(),
                })
            }

            cmd => Err(format!("Unsupported command: [{}]", cmd)),
        }
    }
}
