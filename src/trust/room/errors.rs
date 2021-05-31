use crate::trust::server::{handlers::IncomingChatMessage, ChatServerError};
use actix::prelude::SendError;

#[derive(Debug)]
pub enum ChatRoomError {
    NoServer,
    InvalidUserId(String),
    DuplicateSessionId(String),
    FailedToSend(SendError<IncomingChatMessage>),
}

impl From<ChatRoomError> for ChatServerError {
    fn from(error: ChatRoomError) -> Self {
        ChatServerError::ChatRoomError(error)
    }
}

impl From<SendError<IncomingChatMessage>> for ChatRoomError {
    fn from(error: SendError<IncomingChatMessage>) -> Self {
        ChatRoomError::FailedToSend(error)
    }
}
