use crate::trust::room::ChatRoomError;

#[derive(Debug)]
pub enum ChatServerError {
    ChatRoomError(ChatRoomError),
}
