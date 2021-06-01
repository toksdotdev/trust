mod connect_contract;
mod disconnect_contract;
mod room_contract;

pub use self::{connect_contract::*, disconnect_contract::*, room_contract::*};

/// Chat server sends this messages to session
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct PlainTextMessage(pub String);
