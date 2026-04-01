# rs-clip-bridge-client

[![crates.io](https://img.shields.io/crates/v/rs-clip-bridge-client)](https://crates.io/crates/rs-clip-bridge-client)
[![docs.rs](https://docs.rs/rs-clip-bridge-client/badge.svg)](https://docs.rs/rs-clip-bridge-client)
[![codecov][codecov-src]][codecov-href]
[![License][license-src]][license-href]
[![zread](https://img.shields.io/badge/Ask_Zread-_.svg?style=flat&color=00b0aa&labelColor=000000&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB3aWR0aD0iMTYiIGhlaWdodD0iMTYiIHZpZXdCb3g9IjAgMCAxNiAxNiIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTQuOTYxNTYgMS42MDAxSDIuMjQxNTZDMS44ODgxIDEuNjAwMSAxLjYwMTU2IDEuODg2NjQgMS42MDE1NiAyLjI0MDFWNC45NjAxQzEuNjAxNTYgNS4zMTM1NiAxLjg4ODEgNS42MDAxIDIuMjQxNTYgNS42MDAxSDQuOTYxNTZDNS4zMTUwMiA1LjYwMDEgNS42MDE1NiA1LjMxMzU2IDUuNjAxNTYgNC45NjAxVjIuMjQwMUM1LjYwMTU2IDEuODg2NjQgNS4zMTUwMiAxLjYwMDEgNC45NjE1NiAxLjYwMDFaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik00Ljk2MTU2IDEwLjM5OTlIMi4yNDE1NkMxLjg4ODEgMTAuMzk5OSAxLjYwMTU2IDEwLjY4NjQgMS42MDE1NiAxMS4wMzk5VjEzLjc1OTlDMS42MDE1NiAxNC4xMTM0IDEuODg4MSAxNC4zOTk5IDIuMjQxNTYgMTQuMzk5OUg0Ljk2MTU2QzUuMzE1MDIgMTQuMzk5OSA1LjYwMTU2IDE0LjExMzQgNS42MDE1NiAxMy43NTk5VjExLjAzOTlDNS42MDE1NiAxMC42ODY0IDUuMzE1MDIgMTAuMzk5OSA0Ljk2MTU2IDEwLjM5OTlaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik0xMy43NTg0IDEuNjAwMUgxMS4wMzg0QzEwLjY4NSAxLjYwMDEgMTAuMzk4NCAxLjg4NjY0IDEwLjM5ODQgMi4yNDAxVjQuOTYwMUMxMC4zOTg0IDUuMzEzNTYgMTAuNjg1IDUuNjAwMSAxMS4wMzg0IDUuNjAwMUgxMy43NTg0QzE0LjExMTkgNS42MDAxIDE0LjM5ODQgNS4zMTM1NiAxNC4zOTg0IDQuOTYwMVYyLjI0MDFDMTQuMzk4NCAxLjg4NjY0IDE0LjExMTkgMS42MDAxIDEzLjc1ODQgMS42MDAxWiIgZmlsbD0iI2ZmZiIvPgo8cGF0aCBkPSJNNCAxMkwxMiA0TDQgMTJaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik00IDEyTDEyIDQiIHN0cm9rZT0iI2ZmZiIgc3Ryb2tlLXdpZHRoPSIxLjUiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIvPgo8L3N2Zz4K&logoColor=ffffff)](https://zread.ai/kiki-kanri/rs-clip-bridge)

Cross-platform clipboard sync client for rs-clip-bridge. Monitors local clipboard changes and synchronizes them with a remote server via WebSockets. All clipboard data is encrypted using ChaCha20-Poly1305.

- [✨ Release Notes](./CHANGELOG.md)

## Features

- **Clipboard Monitoring** — Continuously monitors local clipboard for changes using platform-native APIs
- **End-to-End Encryption** — Encrypts clipboard content with ChaCha20-Poly1305 before transmission
- **Circular Write Prevention** — Detects and skips local clipboard writes to avoid feedback loops
- **Multiple Content Types** — Supports text, images, and raw binary data
- **Signal Handling** — Graceful shutdown on SIGINT/SIGTERM
- **Configuration** — Supports TOML config files and environment variables

## Installation

### Pre-built Binaries

Download from the [Latest Release](https://github.com/kiki-kanri/rs-clip-bridge/releases/latest):

```bash
# Linux x86_64 (gnu)
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-x86_64-unknown-linux-gnu -o rs-clip-bridge-client

# Linux x86_64 (musl)
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-x86_64-unknown-linux-musl -o rs-clip-bridge-client

# Linux ARM64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-aarch64-unknown-linux-gnu -o rs-clip-bridge-client

# MacOS x86_64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-x86_64-apple-darwin -o rs-clip-bridge-client

# MacOS ARM64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-aarch64-apple-darwin -o rs-clip-bridge-client

# Windows x86_64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-x86_64-pc-windows-msvc.exe -o rs-clip-bridge-client.exe

# Windows ARM64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-aarch64-pc-windows-msvc.exe -o rs-clip-bridge-client.exe
```

### Build from Source

Requires Rust 1.82+ and platform-specific build dependencies:

- **Linux**: X11 development libraries (`libx11-dev`)
- **macOS**: Xcode command line tools
- **Windows**: Visual Studio Build Tools

TLS backend is selected via feature flag:
- **Default (`rustls-ring`)**: Pure Rust TLS via `ring` crypto
- **`rustls-aws-lc-rs`**: Uses AWS LC-RS crypto backend

```bash
# Default (ring backend)
cargo b -r -p rs-clip-bridge-client

# With aws-lc-rs backend
cargo b -r -p rs-clip-bridge-client --features rustls-aws-lc-rs
```

### Cargo Install

```bash
cargo install rs-clip-bridge-client
```

## Usage

### Quick Start

```bash
rs-clip-bridge-client \
  --server-url ws://127.0.0.1:8000 \
  --channel-id my-channel \
  --encrypt-key 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
```

### Configuration File

Generate a template:

```bash
rs-clip-bridge-client generate-config-template > config.toml
```

Edit `config.toml`:

```toml
server_url = "ws://127.0.0.1:8000"
channel_id = "my-channel"
encrypt_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"

# Optional settings with defaults
# max_image_size_bytes = 10485760  # 10 MB, maximum image size to sync
# min_compress_size_bytes = 1024   # 1 KB, minimum size to trigger compression
```

Run with config:

```bash
rs-clip-bridge-client --config config.toml
```

### Command Line Options

| Option | Description |
|--------|-------------|
| `--server-url` | WebSocket server URL (e.g., `ws://localhost:8000`) |
| `--channel-id` | Channel ID for clipboard isolation |
| `--encrypt-key` | Encryption key (64 hex chars / 32 bytes) |
| `--auth-key` | Authentication key for server access |
| `--display` | X11 display name (Linux only, e.g., `:0`) |
| `--max-image-size-bytes` | Maximum image size to sync (default: 10485760) |
| `--min-compress-size-bytes` | Minimum size to trigger compression (default: 1024) |
| `--config` | Path to TOML configuration file |
| `generate-config-template` | Generate a configuration file template |

### Environment Variables

| Variable | Description |
|----------|-------------|
| `RS_CLIP_SERVER_URL` | WebSocket server URL |
| `RS_CLIP_CHANNEL_ID` | Channel ID for clipboard isolation |
| `RS_CLIP_ENCRYPT_KEY` | Encryption key (64 hex chars / 32 bytes) |
| `RS_CLIP_AUTH_KEY` | Authentication key for server access |
| `RS_CLIP_DISPLAY` | X11 display name (Linux only) |
| `RS_CLIP_MAX_IMAGE_SIZE_BYTES` | Maximum image size to sync |
| `RS_CLIP_MIN_COMPRESS_SIZE_BYTES` | Minimum size to trigger compression |
| `RS_CLIP_CLIENT_CONFIG` | Path to configuration file |

## Security

All clipboard data is encrypted using ChaCha20-Poly1305 before leaving the client. The server never sees unencrypted clipboard content.

### Key Generation

Generate a secure encryption key:

```bash
openssl rand -hex 32
```

## License

[MIT License](../../LICENSE)

<!-- Badges -->
[codecov-href]: https://codecov.io/gh/kiki-kanri/rs-clip-bridge
[codecov-src]: https://codecov.io/gh/kiki-kanri/rs-clip-bridge/graph/badge.svg?token=qvKr7Odjob

[license-href]: https://github.com/kiki-kanri/rs-clip-bridge/blob/main/LICENSE
[license-src]: https://img.shields.io/github/license/kiki-kanri/rs-clip-bridge?colorA=18181b&colorB=28cf8d&style=flat
