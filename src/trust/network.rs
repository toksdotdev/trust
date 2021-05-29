use std::time::Duration;

use actix::{clock::Instant, Actor, ActorContext, Addr, AsyncContext, Running, StreamHandler};
use actix_web_actors::ws;
use ws::WebsocketContext;

use super::server::ChatServer;

pub(crate) struct TrustNetwork {
    last_heartbeat_time: Instant,
    chat_server_address: Addr<ChatServer>,
}

impl TrustNetwork {
    /// How often heartbeat pings are sent
    const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

    /// How long before lack of client response causes a timeout
    const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

    // Create a new instance of trust network.
    pub fn new(chat_server_address: Addr<ChatServer>) -> Self {
        Self {
            last_heartbeat_time: Instant::now(),
            chat_server_address,
        }
    }

    /// Start process to check network heartbeat at interval.
    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(TrustNetwork::HEARTBEAT_INTERVAL, |network, ctx| {
            let time_diff = Instant::now().duration_since(network.last_heartbeat_time);

            if time_diff > TrustNetwork::CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for TrustNetwork {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // TODO: Add support for this.
        // notify chat server
        // self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TrustNetwork {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            _ => {
                println!("Received unsupported message type");
            }
        }
    }
}
