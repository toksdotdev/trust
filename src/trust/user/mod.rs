mod contracts;

use self::contracts::UserContract;
use super::{
    codec::TrustTcpCodec,
    response::error_message,
    server::{
        contracts::{RoomContract, ConnectContract, DisconnectContract, PlainTextMessage},
        TrustServer,
    },
};
use crate::log;
use actix::{
    clock::Instant,
    fut,
    io::{FramedWrite, WriteHandler},
    Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner,
    Handler, Running, StreamHandler, WrapFuture,
};
use std::{
    io::{self, ErrorKind},
    time::Duration,
};
use tokio::io::WriteHalf;
use tokio::net::TcpStream;

pub struct User {
    id: Option<String>,
    last_heartbeat_time: Instant,
    chat_server: Addr<TrustServer>,
    framed: FramedWrite<String, WriteHalf<TcpStream>, TrustTcpCodec>,
}

impl User {
    /// How often heartbeat pings are sent
    const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);

    /// How long before lack of client response causes a timeout
    const CLIENT_TIMEOUT: Duration = Duration::from_secs(300);

    // Create a new instance of user.
    pub fn new(
        chat_server_address: Addr<TrustServer>,
        framed: FramedWrite<String, WriteHalf<TcpStream>, TrustTcpCodec>,
    ) -> Self {
        Self {
            id: None,
            last_heartbeat_time: Instant::now(),
            chat_server: chat_server_address,
            framed,
        }
    }

    /// Start process to check ping user at interval.
    fn heartbeat(&self, ctx: &mut Context<Self>) {
        ctx.run_interval(User::HEARTBEAT_INTERVAL, |user, ctx| {
            let time_diff = Instant::now().duration_since(user.last_heartbeat_time);
            if time_diff <= User::CLIENT_TIMEOUT {
                return user.framed.write("".to_string());
            }

            if let Some(user_id) = &user.id {
                log!("Disconnecting user [{}] after heartbeat failed!", user_id);

                user.chat_server.do_send(DisconnectContract {
                    user_id: user_id.to_string(),
                });
            }

            ctx.stop();
            return;
        });
    }

    // Attempt to register client session to the chat server.
    fn connect_to_chat_server(&self, ctx: &mut Context<Self>) {
        let connect_req = ConnectContract {
            addr: ctx.address().recipient(),
        };

        self.chat_server
            .send(connect_req)
            .into_actor(self)
            .then(|response, user, ctx| {
                if let Ok(Ok(id)) = response {
                    user.id.replace(id);
                    return fut::ready(());
                }

                ctx.stop();
                fut::ready(())
            })
            .wait(ctx);
    }

    /// Handle a message received from a client.
    fn handle_message(&mut self, message: String, _: &mut Context<Self>) {
        if let Ok(cmd) = message.parse::<UserContract>() {
            if let Some(cmd) = self.map_to_server_command(cmd, &message) {
                return self.chat_server.do_send(cmd);
            }
        }

        return self.framed.write(error_message());
    }

    /// Map a chat session command to a chat server command
    fn map_to_server_command(&self, cmd: UserContract, message: &str) -> Option<RoomContract> {
        let cmd = match cmd {
            UserContract::JoinRoom {
                room_name,
                username,
            } => RoomContract::Join {
                user_id: self.id.clone()?,
                room_name: room_name.to_string(),
                username: username.to_string(),
                raw: message.to_string(),
            },

            UserContract::BroadcastMessage(content) => RoomContract::BroadcastMessage {
                user_id: self.id.clone()?,
                content,
            },
        };

        Some(cmd)
    }

    /// Disconnect a c
    fn disconnect(&self) {
        if let Some(ref user_id) = self.id {
            let disconnect_msg = DisconnectContract {
                user_id: user_id.clone(),
            };

            self.chat_server.do_send(disconnect_msg);
        }
    }
}

impl Actor for User {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        self.connect_to_chat_server(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.disconnect();
        Running::Stop
    }
}

/// Handler message coming from the user in context.
impl StreamHandler<Result<String, io::Error>> for User {
    fn handle(&mut self, msg: Result<String, io::Error>, ctx: &mut Context<Self>) {
        self.last_heartbeat_time = Instant::now();

        match msg {
            Ok(text) => self.handle_message(text, ctx),
            Err(err) => {
                log!("Error occurred {:?}", err.kind());
                if err.kind() == ErrorKind::Other {
                    ctx.stop();
                }
            }
        }
    }
}

/// Handle messages from chat server; we simply send it to peer websocket
impl Handler<PlainTextMessage> for User {
    type Result = ();

    fn handle(&mut self, msg: PlainTextMessage, _: &mut Self::Context) {
        self.framed.write(msg.0 + "\n");
    }
}

impl WriteHandler<io::Error> for User {}
