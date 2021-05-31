mod errors;
pub mod handlers;
pub mod utils;

pub use self::errors::*;
use self::handlers::IncomingChatMessage;
use crate::trust::room::{ChatRoom, ChatRoomError};
use actix::{Actor, Context, Recipient};
use parking_lot::RwLock;
use std::{collections::HashMap, rc::Weak};
use uuid::Uuid;

/// User session identifier.
pub type UserSessionId = String;

/// Chat room name.
pub type ChatRoomName = String;

/// Directory of chat server users.
type ChatServerUsers =
    HashMap<UserSessionId, (Recipient<IncomingChatMessage>, Option<ChatRoomName>)>;

#[derive(Debug)]
pub struct ChatServer {
    users: RwLock<ChatServerUsers>,
    rooms: RwLock<HashMap<ChatRoomName, ChatRoom>>,
}

impl ChatServer {
    fn handle_new_connection(
        &mut self,
        client: Recipient<IncomingChatMessage>,
    ) -> Result<String, ChatServerError> {
        // TODO: Hopefully this scales to billions of users to have colliding uuids ;)
        let user_id = Uuid::new_v4().to_string();
        self.users.write().insert(user_id.clone(), (client, None));
        Ok(user_id)
    }

    /// Get all users in the chat server.
    pub(crate) fn get_users(&self) -> &RwLock<ChatServerUsers> {
        &self.users
    }

    /// Check if a user has already joined a room.
    fn get_user_room(&self, user_id: &str) -> Option<String> {
        let lock = self.users.read();

        if let Some((_, Some(room_name))) = lock.get(user_id) {
            return Some(room_name.clone());
        }

        None
    }

    /// Check if a user has already joined a room.
    fn get_username(&self, user_id: &str) -> Option<String> {
        let lock = self.users.read();

        if let Some((_, Some(room_name))) = lock.get(user_id) {
            if let Some(room) = self.rooms.read().get(room_name) {
                return room.get_username(user_id);
            }
        }

        None
    }

    /// Send a direct message to a user.
    fn message_user(&self, user_id: &str, message: &str) {
        if let Some((recipient, _)) = self.users.read().get(user_id) {
            let _ = recipient.do_send(IncomingChatMessage(message.to_string()));
        }
    }

    // Add a user to a room.
    fn add_user_to_room(
        &self,
        room_name: &str,
        user_id: &str,
        username: &str,
    ) -> Result<(), ChatRoomError> {
        let has_address = self.users.read().contains_key(user_id);
        if !has_address {
            return Err(ChatRoomError::InvalidUserId(user_id.to_string()));
        }

        {
            unsafe {
                let server_ptr = Weak::from_raw(self as *const Self);
                self.rooms
                    .write()
                    .entry(room_name.to_string())
                    .or_insert(ChatRoom::new(server_ptr))
                    .add(user_id, username)?;
            }
        }

        self.users
            .write()
            .get_mut(user_id)
            .ok_or(ChatRoomError::InvalidUserId(user_id.to_string()))?
            .1 = Some(room_name.to_string());

        Ok(())
    }

    /// Evict user completely from the server by deleting every record
    /// of the user (including socket connection).
    fn evict_user_from_server(&self, user_id: &str) {
        self.remove_user_active_room(user_id);
        self.users.write().remove(user_id);
    }

    /// Remove user from their currently active room.
    fn remove_user_active_room(&self, user_id: &str) {
        if let Some((_, Some(room_name))) = self.users.read().get(user_id) {
            if let Some(room) = self.rooms.read().get(room_name) {
                room.remove(user_id);

                if room.is_empty() {
                    self.rooms.write().remove(room_name);
                }
            }
        }
    }

    /// Broadcast message to the room of a user.
    fn broadcast_to_room_of_user(&self, user_id: &str, message: &str, exclude_user_ids: &[&str]) {
        if let Some(room_name) = self.get_user_room(user_id) {
            self.broadcast_to_room(&room_name, message, exclude_user_ids);
        }
    }

    /// Broadcast a message to all members of a room.
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
