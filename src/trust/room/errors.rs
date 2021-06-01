use crate::trust::server::{contracts::PlainTextMessage, TrustServerError};
use actix::prelude::SendError;

/// Chat room Error.
#[derive(Debug)]
pub enum RoomError {
    NoServer,
    InvalidUserId(String),
    DuplicateSessionId(String),
    FailedToSend(SendError<PlainTextMessage>),
}

impl From<RoomError> for TrustServerError {
    fn from(error: RoomError) -> Self {
        TrustServerError::RoomError(error)
    }
}

impl From<SendError<PlainTextMessage>> for RoomError {
    fn from(error: SendError<PlainTextMessage>) -> Self {
        RoomError::FailedToSend(error)
    }
}
