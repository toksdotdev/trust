mod contracts;

use self::contracts::UserCommand;
use super::{
    codec::TrustTcpChatCodec,
    server::{
        handlers::{ChatRoomCommand, Connect, Disconnect, IncomingChatMessage},
        ChatServer,
    },
};
use actix::{
    clock::Instant,
    fut,
    io::{FramedWrite, WriteHandler},
    Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner,
    Handler, Running, StreamHandler, WrapFuture,
};
use std::{io, time::Duration};
use tokio::io::WriteHalf;
use tokio::net::TcpStream;

pub struct User {
    id: Option<String>,
    last_heartbeat_time: Instant,
    chat_server: Addr<ChatServer>,
    framed: FramedWrite<String, WriteHalf<TcpStream>, TrustTcpChatCodec>,
}

impl User {
    /// How often heartbeat pings are sent
    const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);

    /// How long before lack of client response causes a timeout
    const CLIENT_TIMEOUT: Duration = Duration::from_secs(300);

    // Create a new instance of user.
    pub fn new(
        chat_server_address: Addr<ChatServer>,
        framed: FramedWrite<String, WriteHalf<TcpStream>, TrustTcpChatCodec>,
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
                println!("Disconnecting user [{}] after heartbeat failed!", user_id);

                user.chat_server.do_send(Disconnect {
                    user_id: user_id.to_string(),
                });
            }

            ctx.stop();
            return;
        });
    }

    // Attempt to register client session to the chat server.
    fn connect_to_chat_server(&self, ctx: &mut Context<Self>) {
        let connect_req = Connect {
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
        if let Ok(cmd) = message.parse::<UserCommand>() {
            if let Some(cmd) = self.map_to_server_command(cmd, &message) {
                return self.chat_server.do_send(cmd);
            }
        }

        return self.framed.write("ERROR<NL>".to_string());
    }

    /// Map a chat session command to a chat server command
    fn map_to_server_command(&self, cmd: UserCommand, message: &str) -> Option<ChatRoomCommand> {
        let cmd = match cmd {
            UserCommand::JoinChatRoom {
                room_name,
                username,
            } => ChatRoomCommand::Join {
                user_id: self.id.clone()?,
                room_name: room_name.to_string(),
                username: username.to_string(),
                raw: message.to_string(),
            },

            UserCommand::BroadcastMessage(content) => ChatRoomCommand::BroadcastMessage {
                user_id: self.id.clone()?,
                content,
            },
        };

        Some(cmd)
    }

    /// Disconnect a c
    fn disconnect(&self) {
        if let Some(ref user_id) = self.id {
            let disconnect_msg = Disconnect {
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
            Err(err) => println!("Error occurred {:?}", err.kind()),
        }
    }
}

/// Handle messages from chat server; we simply send it to peer websocket
impl Handler<IncomingChatMessage> for User {
    type Result = ();

    fn handle(&mut self, msg: IncomingChatMessage, _: &mut Self::Context) {
        self.framed.write(msg.0 + "\n");
    }
}

impl WriteHandler<io::Error> for User {}
