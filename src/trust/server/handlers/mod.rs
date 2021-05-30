mod connect_handler;
mod disconnect_handler;
mod room_handler;

pub use self::{connect_handler::*, disconnect_handler::*, room_handler::*};

/// Chat server sends this messages to session
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Message(pub String);
