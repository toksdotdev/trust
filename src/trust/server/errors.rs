use crate::trust::room::RoomError;

#[derive(Debug)]
pub enum TrustServerError {
    RoomError(RoomError),
}
