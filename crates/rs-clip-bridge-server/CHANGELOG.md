# Changelog

## 0.1.2 - 2026-03-26 02:11

[compare changes](https://github.com/kiki-kanri/rs-clip-bridge/compare/rs-clip-bridge-server-v0.1.1...rs-clip-bridge-server-v0.1.2)

### 💅 Refactors

- replace `map_err` and `ok_or_else` with `context` ([5a1a959](https://github.com/ws-io/ws.io-rs/commit/5a1a959))

## 0.1.1 - 2026-03-25 19:10

[compare changes](https://github.com/kiki-kanri/rs-clip-bridge/compare/rs-clip-bridge-server-v0.1.0...rs-clip-bridge-server-v0.1.1)

### 🏡 Chore

- refine tracing log levels and add more logs ([bc8433c](https://github.com/ws-io/ws.io-rs/commit/bc8433c))

### 📖 Documentation

- update README ([1bc8350](https://github.com/ws-io/ws.io-rs/commit/1bc8350))

## 0.1.0 - 2026-03-25 17:41

[compare changes](https://github.com/kiki-kanri/rs-clip-bridge/releases/tag/rs-clip-bridge-server-v0.1.0)

### 🏡 Chore

- lint code ([d32fc11](https://github.com/ws-io/ws.io-rs/commit/d32fc11))

### 💅 Refactors

- update ([5ffdf36](https://github.com/ws-io/ws.io-rs/commit/5ffdf36))
- *(rs-clip-bridge-server)* rename `AppConfig` to `ServerConfig` ([59951e4](https://github.com/ws-io/ws.io-rs/commit/59951e4))

### 📖 Documentation

- update readme ([a181f32](https://github.com/ws-io/ws.io-rs/commit/a181f32))
- add readme ([2e27f4d](https://github.com/ws-io/ws.io-rs/commit/2e27f4d))

### 🚀 Enhancements

- allow use env set config path ([f6f9ee2](https://github.com/ws-io/ws.io-rs/commit/f6f9ee2))
- feat!(server): change auth key config to array ([81fc347](https://github.com/ws-io/ws.io-rs/commit/81fc347))
- add command to generate config template ([6cb8ecb](https://github.com/ws-io/ws.io-rs/commit/6cb8ecb))
- E2EE and refactor codes ([5b89e83](https://github.com/ws-io/ws.io-rs/commit/5b89e83))
- initial completion of functions ([d54906a](https://github.com/ws-io/ws.io-rs/commit/d54906a))
- *(rs-clip-bridge-server)* basic functions have been initially completed ([413fef6](https://github.com/ws-io/ws.io-rs/commit/413fef6))
- add create rs-clip-bridge-server crate and add base files ([5ac06f7](https://github.com/ws-io/ws.io-rs/commit/5ac06f7))

### 🤖 CI

- remove sha image tag and set docker image labels ([dae559c](https://github.com/ws-io/ws.io-rs/commit/dae559c))
- update dockerfile and workflow file ([062d0be](https://github.com/ws-io/ws.io-rs/commit/062d0be))
- add server dockerfile and related ci files to release docker image ([0dfc0aa](https://github.com/ws-io/ws.io-rs/commit/0dfc0aa))

### 🩹 Fixes

- *(server)* set config default value ([48d410f](https://github.com/ws-io/ws.io-rs/commit/48d410f))
- *(server)* remove copy `rustfmt.toml` ([cfc8dd5](https://github.com/ws-io/ws.io-rs/commit/cfc8dd5))
