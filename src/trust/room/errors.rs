use crate::trust::server::{handlers::Message, ChatServerError};
use actix::prelude::SendError;

#[derive(Debug)]
pub enum ChatRoomError {
    NoServer,
    InvalidUserId(String),
    DuplicateSessionId(String),
    FailedToSend(SendError<Message>),
}

impl From<ChatRoomError> for ChatServerError {
    fn from(error: ChatRoomError) -> Self {
        ChatServerError::ChatRoomError(error)
    }
}

impl From<SendError<Message>> for ChatRoomError {
    fn from(error: SendError<Message>) -> Self {
        ChatRoomError::FailedToSend(error)
    }
}
