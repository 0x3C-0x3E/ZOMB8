# ZOMB8
Multiplayer arena zombie shooter using the UDP transport layer protocl and the hecs ECS-System. Written in rust btw

## Building

> [!WARNING]
> see [Configuration](#Configuration) on how to change the server IP
### Client
```bash
cargo build -p client --release
```
### Server
```bash
cargo build -p server --release
```

## Configuration
The config file is located in assets/config.toml

Default Config file:
```
server_ip = "192.168.2.119"

port = 6969
tps = 20
```
