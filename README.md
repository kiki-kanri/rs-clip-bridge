# rs-clip-bridge

[![codecov][codecov-src]][codecov-href]
[![License][license-src]][license-href]

A blazingly fast, cross-platform clipboard synchronizer using WebSockets with end-to-end ChaCha20-Poly1305 encryption. Sync clipboard content across multiple devices in real-time.

## Features

- **Real-time Sync** — Clipboard changes propagate instantly across all connected devices via WebSockets
- **End-to-End Encryption** — All clipboard data is encrypted using ChaCha20-Poly1305 before transmission
- **Channel Isolation** — Devices are organized into channels, ensuring clipboard data only syncs within designated groups
- **Cross-Platform** — Works on Linux (X11), macOS, and Windows
- **Multiple Data Types** — Supports text and binary clipboard content (image support planned)
- **WebSocket Transport** — Reliable, connection-oriented communication with automatic reconnection

## Architecture

```
rs-clip-bridge
├── crates/
│   ├── rs-clip-bridge-client/   # Clipboard monitoring client
│   ├── rs-clip-bridge-server/   # WebSocket relay server
│   └── rs-clip-bridge-types/     # Shared data types
```

## Installation

### Prerequisites

- Rust 1.82+ (Edition 2024)
- Cargo

### Build from Source

```bash
git clone https://github.com/kiki-kanri/rs-clip-bridge.git
cd rs-clip-bridge
cargo b -r
```

Binaries will be available at `target/release/rs-clip-bridge-client` and `target/release/rs-clip-bridge-server`.

### Cargo Install

```bash
cargo install rs-clip-bridge-client
cargo install rs-clip-bridge-server
```

### Docker

```bash
# Build server image
docker build -t rs-clip-bridge-server -f crates/rs-clip-bridge-server/Dockerfile .
```

### GitHub Actions

Pre-built binaries are available from the [Auto compile workflow](https://github.com/kiki-kanri/rs-clip-bridge/actions/workflows/auto-compile.yaml):

```bash
# Download from workflow artifacts
# Select a run → Artifacts → download bin-<target>
```

## Usage

### Server

Start the WebSocket server:

```bash
rs-clip-bridge-server --host 127.0.0.1 --port 8000
```

Or use a configuration file:

```bash
rs-clip-bridge-server --config /path/to/config.toml
```

Generate a configuration template:

```bash
rs-clip-bridge-server generate-config-template
```

### Client

Connect a client to the server:

```bash
rs-clip-bridge-client \
  --server-url ws://127.0.0.1:8000 \
  --channel-id my-channel \
  --encrypt-key $(openssl rand -hex 32)
```

Or use a configuration file:

```bash
rs-clip-bridge-client --config /path/to/config.toml
```

Generate a configuration template:

```bash
rs-clip-bridge-client generate-config-template
```

### Configuration

Both client and server support configuration via TOML files and environment variables.

#### Client Environment Variables

| Variable | Description |
|----------|-------------|
| `RS_CLIP_AUTH_KEY` | Authentication key for server access |
| `RS_CLIP_CHANNEL_ID` | Channel ID for clipboard isolation |
| `RS_CLIP_ENCRYPT_KEY` | Encryption key (64 hex chars / 32 bytes) |
| `RS_CLIP_SERVER_URL` | WebSocket server URL |
| `RS_CLIP_DISPLAY` | X11 display name (Linux only) |

#### Server Environment Variables

| Variable | Description |
|----------|-------------|
| `RS_CLIP_AUTH_KEYS` | Comma-separated list of auth keys |
| `RS_CLIP_SERVER_HOST` | Server bind address |
| `RS_CLIP_SERVER_PORT` | Server port |

## License

[MIT License](./LICENSE)

<!-- Badges -->
[codecov-href]: https://codecov.io/gh/kiki-kanri/rs-clip-bridge
[codecov-src]: https://codecov.io/gh/kiki-kanri/rs-clip-bridge/graph/badge.svg?token=qvKr7Odjob

[license-href]: https://github.com/kiki-kanri/rs-clip-bridge/blob/main/LICENSE
[license-src]: https://img.shields.io/github/license/kiki-kanri/rs-clip-bridge?colorA=18181b&colorB=28cf8d&style=flat
