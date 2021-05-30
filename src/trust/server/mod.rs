mod command;
pub mod handlers;
pub mod utils;

use self::handlers::{JoinChatRoom, Message};
use super::room::{ChatRoom, ChatRoomError};
use actix::{Actor, Context, Recipient};
use parking_lot::RwLock;
use std::{collections::HashMap, rc::Weak};
use uuid::Uuid;

pub type UserSessionId = String;
pub type ChatRoomName = String;

#[derive(Debug)]
pub struct ChatServer {
    users: RwLock<HashMap<UserSessionId, Recipient<Message>>>,
    rooms: RwLock<HashMap<ChatRoomName, ChatRoom>>,
}

#[derive(Debug)]
pub enum ChatServerError {
    ChatRoomError(ChatRoomError),
}

impl ChatServer {
    fn handle_new_connection(
        &mut self,
        client: Recipient<Message>,
    ) -> Result<String, ChatServerError> {
        // TODO: Hopefully this scales to billions of users to have colliding uuids ;)
        let user_id = Uuid::new_v4().to_string();
        self.users.write().insert(user_id.clone(), client).unwrap();
        Ok(user_id)
    }

    /// Get all users in the chat server.
    pub(crate) fn get_users(&self) -> &RwLock<HashMap<UserSessionId, Recipient<Message>>> {
        &self.users
    }

    // Add a user to a room.
    fn add_user_to_room(&self, payload: &JoinChatRoom) -> Result<(), ChatRoomError> {
        let has_address = self.users.write().contains_key(&payload.user_id);
        if !has_address {
            return Err(ChatRoomError::InvalidUserId(payload.user_id.clone()));
        }

        unsafe {
            let server_ptr = Weak::from_raw(self as *const Self);
            self.rooms
                .write()
                .entry(payload.room_name.to_string())
                .or_insert(ChatRoom::new(server_ptr))
                .add(&payload.user_id, &payload.username)?;
        }

        Ok(())
    }

    /// Remove user from all rooms.
    fn remove_user_from_all_rooms(&self, user_id: &str) {
        for room in &mut self.rooms.read().values() {
            room.remove(user_id);
        }
    }

    /// Broadcast a message to all rooms excluding the user ids specified.
    fn broadcast_to_all_rooms(&self, message: &str, exclude_user_ids: &[&str]) {
        for room_name in self.rooms.read().keys() {
            self.broadcast_to_room(&room_name, message, exclude_user_ids);
        }
    }

    /// Broadcast a message to a rooms excluding the session ids specified.
    fn broadcast_to_room(&self, room_name: &str, message: &str, exclude_user_ids: &[&str]) {
        if let Some(chat_room) = self.rooms.read().get(room_name) {
            if let Err(err) = chat_room.broadcast_to_excluding(message, exclude_user_ids) {
                println!(
                    "Error occurred while sending message to room: [{}]; error: [{:?}]",
                    room_name, err
                )
            }
        }
    }
}

impl Default for ChatServer {
    fn default() -> Self {
        ChatServer {
            rooms: RwLock::default(),
            users: RwLock::default(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}
