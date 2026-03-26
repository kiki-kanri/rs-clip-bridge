# rs-clip-bridge-server

[![crates.io](https://img.shields.io/crates/v/rs-clip-bridge-server)](https://crates.io/crates/rs-clip-bridge-server)
[![docs.rs](https://docs.rs/rs-clip-bridge-server/badge.svg)](https://docs.rs/rs-clip-bridge-server)
[![codecov][codecov-src]][codecov-href]
[![License][license-src]][license-href]

WebSocket server for rs-clip-bridge clipboard synchronization. Relays encrypted clipboard events between clients within the same channel.

- [✨ Release Notes](./CHANGELOG.md)

## Features

- **WebSocket Transport** — Built on wsio-server for reliable, connection-oriented communication
- **Channel-based Routing** — Clients join channels and only receive events from their channel
- **Authentication** — Supports configurable auth keys to control access
- **Tower Integration** — Runs as an Axum middleware layer for easy integration
- **Graceful Shutdown** — Handles SIGINT/SIGTERM for clean shutdown
- **Postcard Codec** — Efficient binary serialization using the Postcard format

## Installation

### Pre-built Binaries

Download from the [Latest Release](https://github.com/kiki-kanri/rs-clip-bridge/releases/latest):

```bash
# Linux x86_64 (gnu)
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-x86_64-unknown-linux-gnu -o rs-clip-bridge-server

# Linux x86_64 (musl)
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-x86_64-unknown-linux-musl -o rs-clip-bridge-server

# Linux ARM64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-aarch64-unknown-linux-gnu -o rs-clip-bridge-server

# Windows x86_64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-x86_64-pc-windows-msvc.exe -o rs-clip-bridge-server.exe

# Windows ARM64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-aarch64-pc-windows-msvc.exe -o rs-clip-bridge-server.exe
```

### Build from Source

Requires Rust 1.82+ and Cargo.

```bash
cargo b -r -p rs-clip-bridge-server
```

### Cargo Install

```bash
cargo install rs-clip-bridge-server
```

### Docker

```bash
# Build image
docker build -t rs-clip-bridge-server -f crates/rs-clip-bridge-server/Dockerfile .

# Run container
docker run -d \
  --name rs-clip-bridge-server \
  -p 8000:8000 \
  -e RS_CLIP_AUTH_KEYS=a,b \
  -e RS_CLIP_SERVER_HOST=0.0.0.0 \
  -e RS_CLIP_SERVER_PORT=8000 \
  rs-clip-bridge-server
```

## Usage

### Quick Start

```bash
rs-clip-bridge-server --host 127.0.0.1 --port 8000
```

### Configuration File

Generate a template:

```bash
rs-clip-bridge-server generate-config-template > config.toml
```

Edit `config.toml`:

```toml
host = "127.0.0.1"
port = 8000
auth_keys = ["my-secret-key"]
```

Run with config:

```bash
rs-clip-bridge-server --config config.toml
```

### Command Line Options

| Option | Description |
|--------|-------------|
| `--host` | Server bind address (default: `127.0.0.1`) |
| `--port` | Server port (default: `8000`) |
| `--auth-keys` | Comma-separated list of valid auth keys |
| `--config` | Path to TOML configuration file |
| `generate-config-template` | Generate a configuration file template |

### Environment Variables

| Variable | Description |
|----------|-------------|
| `RS_CLIP_SERVER_HOST` | Server bind address |
| `RS_CLIP_SERVER_PORT` | Server port |
| `RS_CLIP_AUTH_KEYS` | Comma-separated list of auth keys |
| `RS_CLIP_SERVER_CONFIG` | Path to configuration file |

## Protocol

### Init Handshake

Clients send `(auth_key, channel_id)` during WebSocket connection initialization.

### Events

The server relays `ClipboardEventData` messages between clients in the same channel:

```rust
struct ClipboardEventData {
    device_name: Option<String>,
    content: Vec<u8>,   // Encrypted: [ciphertext || poly1305_tag]
    nonce: Vec<u8>,     // 12 bytes
    timestamp: u64,     // Unix timestamp in milliseconds
}
```

### Namespace

The server exposes a single namespace at `/` for all clipboard events.

## License

[MIT License](../../LICENSE)

<!-- Badges -->
[codecov-href]: https://codecov.io/gh/kiki-kanri/rs-clip-bridge
[codecov-src]: https://codecov.io/gh/kiki-kanri/rs-clip-bridge/graph/badge.svg?token=qvKr7Odjob

[license-href]: https://github.com/kiki-kanri/rs-clip-bridge/blob/main/LICENSE
[license-src]: https://img.shields.io/github/license/kiki-kanri/rs-clip-bridge?colorA=18181b&colorB=28cf8d&style=flat
