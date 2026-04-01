# rs-clip-bridge

[![codecov][codecov-src]][codecov-href]
[![License][license-src]][license-href]
[![zread](https://img.shields.io/badge/Ask_Zread-_.svg?style=flat&color=00b0aa&labelColor=000000&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB3aWR0aD0iMTYiIGhlaWdodD0iMTYiIHZpZXdCb3g9IjAgMCAxNiAxNiIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTQuOTYxNTYgMS42MDAxSDIuMjQxNTZDMS44ODgxIDEuNjAwMSAxLjYwMTU2IDEuODg2NjQgMS42MDE1NiAyLjI0MDFWNC45NjAxQzEuNjAxNTYgNS4zMTM1NiAxLjg4ODEgNS42MDAxIDIuMjQxNTYgNS42MDAxSDQuOTYxNTZDNS4zMTUwMiA1LjYwMDEgNS42MDE1NiA1LjMxMzU2IDUuNjAxNTYgNC45NjAxVjIuMjQwMUM1LjYwMTU2IDEuODg2NjQgNS4zMTUwMiAxLjYwMDEgNC45NjE1NiAxLjYwMDFaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik00Ljk2MTU2IDEwLjM5OTlIMi4yNDE1NkMxLjg4ODEgMTAuMzk5OSAxLjYwMTU2IDEwLjY4NjQgMS42MDE1NiAxMS4wMzk5VjEzLjc1OTlDMS42MDE1NiAxNC4xMTM0IDEuODg4MSAxNC4zOTk5IDIuMjQxNTYgMTQuMzk5OUg0Ljk2MTU2QzUuMzE1MDIgMTQuMzk5OSA1LjYwMTU2IDE0LjExMzQgNS42MDE1NiAxMy43NTk5VjExLjAzOTlDNS42MDE1NiAxMC42ODY0IDUuMzE1MDIgMTAuMzk5OSA0Ljk2MTU2IDEwLjM5OTlaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik0xMy43NTg0IDEuNjAwMUgxMS4wMzg0QzEwLjY4NSAxLjYwMDEgMTAuMzk4NCAxLjg4NjY0IDEwLjM5ODQgMi4yNDAxVjQuOTYwMUMxMC4zOTg0IDUuMzEzNTYgMTAuNjg1IDUuNjAwMSAxMS4wMzg0IDUuNjAwMUgxMy43NTg0QzE0LjExMTkgNS42MDAxIDE0LjM5ODQgNS4zMTM1NiAxNC4zOTg0IDQuOTYwMVYyLjI0MDFDMTQuMzk4NCAxLjg4NjY0IDE0LjExMTkgMS42MDAxIDEzLjc1ODQgMS42MDAxWiIgZmlsbD0iI2ZmZiIvPgo8cGF0aCBkPSJNNCAxMkwxMiA0TDQgMTJaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik00IDEyTDEyIDQiIHN0cm9rZT0iI2ZmZiIgc3Ryb2tlLXdpZHRoPSIxLjUiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIvPgo8L3N2Zz4K&logoColor=ffffff)](https://zread.ai/kiki-kanri/rs-clip-bridge)

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

### Pre-built Binaries

Download from the [Latest Release](https://github.com/kiki-kanri/rs-clip-bridge/releases/tag/latest):

```bash
# Linux x86_64 (gnu)
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-x86_64-unknown-linux-gnu -o rs-clip-bridge-client
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-x86_64-unknown-linux-gnu -o rs-clip-bridge-server

# Linux x86_64 (musl)
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-x86_64-unknown-linux-musl -o rs-clip-bridge-client
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-x86_64-unknown-linux-musl -o rs-clip-bridge-server

# Linux ARM64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-aarch64-unknown-linux-gnu -o rs-clip-bridge-client
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-aarch64-unknown-linux-gnu -o rs-clip-bridge-server

# MacOS x86_64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-x86_64-apple-darwin -o rs-clip-bridge-client
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-x86_64-apple-darwin -o rs-clip-bridge-server

# MacOS ARM64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-aarch64-apple-darwin -o rs-clip-bridge-client
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-aarch64-apple-darwin -o rs-clip-bridge-server

# Windows x86_64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-x86_64-pc-windows-msvc.exe -o rs-clip-bridge-client.exe
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-x86_64-pc-windows-msvc.exe -o rs-clip-bridge-server.exe

# Windows ARM64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-aarch64-pc-windows-msvc.exe -o rs-clip-bridge-client.exe
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-server-aarch64-pc-windows-msvc.exe -o rs-clip-bridge-server.exe
```

### Docker

```bash
# Build server image
docker build -t rs-clip-bridge-server -f crates/rs-clip-bridge-server/Dockerfile .
```

### Build from Source

Requires Rust 1.82+ and Cargo.

```bash
git clone https://github.com/kiki-kanri/rs-clip-bridge.git
cd rs-clip-bridge
cargo b -r
```

Binaries will be available at `target/release/rs-clip-bridge-client` and `target/release/rs-clip-bridge-server`.

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
