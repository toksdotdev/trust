# 🔑 Trust Chat

A very basic chat server in Rust.

## Running the example

Open up your terminal, and run the following command.

```bash
cargo run
```

Open up another terminal, and telnet to the corresponding ip and port (default port is `1234`).

```bash
telnet 0.0.0.0 1234
```

## Configuring the port

```bash
cargo run -- --port=9090
```

## Support commands

### To join a room

> NOTE: You can only join only 1 room in the entire client lifetime.

```
JOIN {room_name} {username}
```

### To send message

> NOTE: You'll get an error message if you attempt to send a message without joining a room.

```
Any random message.
```

## Digging Deeper

### Entities

All entities in the system are running as actors, and they include:

- [User](./src/trust/user/mod.rs)
- [Server](./src/trust/server/mod.rs)

### Protocol

The communication between an external client(e.g. telnet) and the server adopts a very simple Codec which can ve found [here](./src/trust/codec.rs).

## Contributing

If you see any area this could be improved upon, kindly feel free to open a PR. Cheers!!!.
