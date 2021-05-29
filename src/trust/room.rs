use std::collections::HashMap;

use crate::trust::contracts::Message;
use actix::prelude::*;
use parking_lot::RwLock;
use uuid::Uuid;

#[rtype(result = "()")]
#[derive(Message, Debug)]
pub(crate) struct ChatRoom(RwLock<HashMap<String, Recipient<Message>>>);

impl ChatRoom {
    /// Add a client to the room (returns a unique session id).
    pub fn add(&self, client: Recipient<Message>) -> String {
        let session_id = Uuid::new_v4().to_string();
        self.0.write().insert(session_id.clone(), client);
        session_id
    }

    // Remove a client from the room.
    pub fn remove(&self, session_id: &str) {
        self.0.write().remove(session_id);
    }

    // Broadcast message to everyone in chat room excluding users specified.
    pub fn broadcast_to_excluding(
        &self,
        message: &str,
        excluding: &[&str],
    ) -> Result<(), SendError<Message>> {
        for (id, client) in self.0.read().iter() {
            if !excluding.contains(&id.as_str()) {
                client.do_send(Message(message.to_owned()))?;
            }
        }

        Ok(())
    }
}

impl Default for ChatRoom {
    fn default() -> Self {
        Self(RwLock::default())
    }
}
