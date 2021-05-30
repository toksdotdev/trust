mod room_handler;
mod connect_handler;
mod disconnect_handler;

pub use self::{room_handler::*, connect_handler::*, disconnect_handler::*};

/// Chat server sends this messages to session
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Message(pub String);
