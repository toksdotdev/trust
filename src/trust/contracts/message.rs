/// Chat server sends this messages to session
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Message(pub String);
