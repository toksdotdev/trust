use super::room::ChatRoom;
use actix::{Actor, Context};
use std::collections::HashMap;

pub struct ChatServer {
    rooms: HashMap<String, ChatRoom>,
}

impl ChatServer {
    fn send_message(&self, room_name: &str, message: &str, exclude: &[&str]) {
        if let Some(chat_room) = self.rooms.get(room_name) {
            if let Err(err) = chat_room.broadcast_to_excluding(message, exclude) {
                println!(
                    "Error occurred while sending message to room: [{}]; error: [{}]",
                    room_name, err
                )
            }
        }
    }
}

impl Default for ChatServer {
    fn default() -> Self {
        let mut rooms = HashMap::new();
        rooms.insert("#general".to_owned(), ChatRoom::default());
        ChatServer { rooms }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}
