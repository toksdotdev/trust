use crate::trust::server::{contracts::PlainTextMessage, ChatServerError};
use actix::prelude::SendError;

/// Chat room Error.
#[derive(Debug)]
pub enum ChatRoomError {
    NoServer,
    InvalidUserId(String),
    DuplicateSessionId(String),
    FailedToSend(SendError<PlainTextMessage>),
}

impl From<ChatRoomError> for ChatServerError {
    fn from(error: ChatRoomError) -> Self {
        ChatServerError::ChatRoomError(error)
    }
}

impl From<SendError<PlainTextMessage>> for ChatRoomError {
    fn from(error: SendError<PlainTextMessage>) -> Self {
        ChatRoomError::FailedToSend(error)
    }
}
