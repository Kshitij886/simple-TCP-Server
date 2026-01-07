# TCP-server

Simple TCP server written in Rust. The server listens on 127.0.0.1:8000 and responds with a static message for each client connection.

## Project Structure

- Cargo.toml — Rust manifest
- src/main.rs — TCP server implementation

## Requirements

- Rust (stable) — install via https://rustup.rs/

## Build

From the project root:

```bash
cargo build --release
```

## Run

Run the server (debug):

```bash
cargo run
```

Or run the release binary after building:

```bash
cargo run --release
# or
cargo build --release
./target/release/TCP-server   # Windows: target\release\TCP-server.exe
```

The server binds to `127.0.0.1:8000` and prints a message when it receives data.

## Example

- Using `telnet` (Windows/macOS/Linux):

```bash
telnet 127.0.0.1 8000
# Type any text and press Enter — the server responds with "hello world "
```

- Using `nc` / netcat:

```bash
echo "ping" | nc 127.0.0.1 8000
```

Expected server output (console):

```
Recieved request: ping
```

Client receives:

```
hello world 
```

## Notes

This is a minimal example to demonstrate handling TCP connections in Rust. `src/main.rs` contains the current implementation and listens on a hard-coded address and port.

