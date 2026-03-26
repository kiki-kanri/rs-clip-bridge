# rs-clip-bridge-client

[![crates.io](https://img.shields.io/crates/v/rs-clip-bridge-client)](https://crates.io/crates/rs-clip-bridge-client)
[![docs.rs](https://docs.rs/rs-clip-bridge-client/badge.svg)](https://docs.rs/rs-clip-bridge-client)
[![codecov][codecov-src]][codecov-href]
[![License][license-src]][license-href]

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

# Windows x86_64
curl -L https://github.com/kiki-kanri/rs-clip-bridge/releases/download/latest/rs-clip-bridge-client-x86_64-pc-windows-msvc.exe -o rs-clip-bridge-client.exe
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
