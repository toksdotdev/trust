mod contracts;

use self::contracts::ChatSessionCommand;
use super::server::{
    handlers::{ChatServerCommand, Connect, Disconnect, JoinChatRoom, Message},
    ChatServer,
};
use actix::{
    clock::Instant, fut, Actor, ActorContext, ActorFuture, Addr, AsyncContext,
    ContextFutureSpawner, Handler, Running, StreamHandler, WrapFuture,
};
use actix_http::ws::Item;
use actix_web_actors::ws;
use parking_lot::Mutex;
use std::time::Duration;
use ws::WebsocketContext;

pub struct ChatSession {
    user_id: Option<String>,
    last_heartbeat_time: Instant,
    chat_server: Addr<ChatServer>,
    buffer: Mutex<Vec<u8>>,
}

impl ChatSession {
    /// How often heartbeat pings are sent
    const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

    /// How long before lack of client response causes a timeout
    const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

    // Create a new instance of trust network.
    pub fn new(chat_server_address: Addr<ChatServer>) -> Self {
        Self {
            user_id: None,
            buffer: Mutex::default(),
            last_heartbeat_time: Instant::now(),
            chat_server: chat_server_address,
        }
    }

    /// Start process to check network heartbeat of user at interval.
    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(ChatSession::HEARTBEAT_INTERVAL, |network, ctx| {
            let time_diff = Instant::now().duration_since(network.last_heartbeat_time);

            if time_diff > ChatSession::CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }

    // Attempt to register client session to the chat server.
    fn connect_to_chat_server(&self, ctx: &mut WebsocketContext<Self>) {
        let connect_req = Connect {
            addr: ctx.address().recipient(),
        };

        self.chat_server
            .send(connect_req)
            .into_actor(self)
            .then(|response, chat_session, ctx| {
                if let Ok(Ok(session_id)) = response {
                    chat_session.user_id.replace(session_id);
                    return fut::ready(());
                }

                ctx.stop();
                fut::ready(())
            })
            .wait(ctx);
    }

    /// Handle a complete message received from a client.
    fn handle_complete_message(&self, text: String, ctx: &mut WebsocketContext<Self>) {
        for message in text.split("<NL>") {
            if let Ok(cmd) = message.parse::<ChatSessionCommand>() {
                return self.chat_server.do_send(self.map_to_server_command(&cmd));
            }

            return ctx.text("ERROR<NL>");
        }
    }

    /// Map a chat session command to a chat server command
    fn map_to_server_command(&self, session_command: &ChatSessionCommand) -> ChatServerCommand {
        match session_command {
            ChatSessionCommand::JoinChatRoom {
                room_name,
                username,
            } => ChatServerCommand::JoinChatRoom(JoinChatRoom {
                user_id: self.user_id.clone().unwrap(),
                room_name: room_name.to_string(),
                username: username.to_string(),
            }),
        }
    }

    //' Handle chunked messages.
    fn handle_chunked_message(&self, item: &Item, ctx: &mut WebsocketContext<Self>) {
        match item {
            Item::FirstText(chunk) | Item::Continue(chunk) => {
                self.buffer.lock().append(&mut chunk.to_vec())
            }

            Item::Last(chunk) => {
                let mut session_buffer = self.buffer.lock();
                session_buffer.append(&mut chunk.to_vec());

                self.handle_complete_message(
                    String::from_utf8_lossy(&session_buffer[..]).to_string(),
                    ctx,
                );

                session_buffer.clear();
            }
            _ => {}
        };
    }

    fn disconnect(&self) {
        //  User only has session ID if they've registered with the serve
        if let Some(ref user_id) = self.user_id {
            let disconnect_msg = Disconnect {
                user_id: user_id.clone(),
            };

            self.chat_server.do_send(disconnect_msg);
        }
    }
}

impl Actor for ChatSession {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        self.connect_to_chat_server(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.disconnect();
        Running::Stop
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        self.last_heartbeat_time = Instant::now();

        match msg {
            Ok(ws::Message::Text(text)) => self.handle_complete_message(text, ctx),
            Ok(ws::Message::Continuation(item)) => self.handle_chunked_message(&item, ctx),
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            _ => println!("Received unsupported message type"),
        }
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<Message> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
