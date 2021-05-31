use crate::trust::server::utils::{valid_chatroom_name, valid_username};
use std::str::FromStr;

#[derive(Debug)]
pub enum UserCommand {
    JoinChatRoom { username: String, room_name: String },
    BroadcastMessage(String),
}

impl FromStr for UserCommand {
    type Err = String;

    fn from_str(message: &str) -> Result<Self, Self::Err> {
        let message = message.replace("\n", "");
        let mut fragments = message.split_ascii_whitespace();
        let command = fragments
            .next()
            .ok_or("Command cannot be empty".to_string())?;

        match command.to_ascii_lowercase().as_str() {
            "join" => {
                let room_name = fragments.next();
                if !valid_chatroom_name(room_name) {
                    return Err("Invalid room name".to_string());
                }

                let username = fragments.next();
                if !valid_username(username) {
                    return Err("Invalid username".to_string());
                }

                if fragments.next().is_some() {
                    return Err("Invalid join command specified".to_string());
                }

                Ok(Self::JoinChatRoom {
                    username: username.unwrap().to_string(),
                    room_name: room_name.unwrap().to_string(),
                })
            }

            _ => Ok(Self::BroadcastMessage(message)),
        }
    }
}
