mod errors;

pub use self::errors::*;
use crate::trust::server::contracts::PlainTextMessage;
use crate::trust::server::{TrustServer, UserSessionId};
use actix::prelude::*;
use parking_lot::RwLock;
use std::{collections::HashMap, rc::Weak};

type Username = String;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Room {
    server: Weak<TrustServer>,
    store: RwLock<HashMap<UserSessionId, Username>>,
}

impl Room {
    pub fn new(server: Weak<TrustServer>) -> Self {
        Self {
            server,
            store: RwLock::default(),
        }
    }

    // Get username of a user in a chatroom.
    pub fn get_username(&self, user_id: &str) -> Option<String> {
        self.store.read().get(user_id).map(|s| s.clone())
    }

    /// Check if chatroom is empty.
    pub fn is_empty(&self) -> bool {
        self.store.read().is_empty()
    }

    /// Add a client to the room.
    pub fn add(&self, user_id: &str, username: &str) -> Result<(), RoomError> {
        if let Some(_) = self
            .store
            .write()
            .insert(user_id.to_string(), username.to_string())
        {
            return Err(RoomError::DuplicateSessionId(user_id.to_string()));
        }

        Ok(())
    }

    // Remove a user from the room.
    pub fn remove(&self, user_id: &str) {
        self.store.write().remove(user_id);
    }

    // Broadcast message to everyone in chat room excluding users specified.
    pub fn broadcast_to_excluding(
        &self,
        message: &str,
        excluding: &[&str],
    ) -> Result<(), RoomError> {
        let server = self.server.upgrade().ok_or(RoomError::NoServer)?;

        self.store
            .read()
            .keys()
            .into_iter()
            .for_each(move |user_id| {
                if !excluding.contains(&user_id.as_str()) {
                    if let Some(address) = server.get_users().read().get(user_id) {
                        let _ = address.0.do_send(PlainTextMessage(message.to_owned()));
                    }
                }
            });

        Ok(())
    }
}
