# Changelog

## 0.1.2 - 2026-03-26 02:11

[compare changes](https://github.com/kiki-kanri/rs-clip-bridge/compare/rs-clip-bridge-client-v0.1.1...rs-clip-bridge-client-v0.1.2)

### 💅 Refactors

- replace `map_err` and `ok_or_else` with `context` ([5a1a959](https://github.com/ws-io/ws.io-rs/commit/5a1a959))

### 🩹 Fixes

- *(client)* address incomplete `cfg` feature flags in `init_rustls_provider` ([9191725](https://github.com/ws-io/ws.io-rs/commit/9191725))

## 0.1.1 - 2026-03-25 19:10

[compare changes](https://github.com/kiki-kanri/rs-clip-bridge/compare/rs-clip-bridge-client-v0.1.0...rs-clip-bridge-client-v0.1.1)

### 🏡 Chore

- refine tracing log levels and add more logs ([bc8433c](https://github.com/ws-io/ws.io-rs/commit/bc8433c))

### 📖 Documentation

- update README ([ef2e7ab](https://github.com/ws-io/ws.io-rs/commit/ef2e7ab))
- update README ([1bc8350](https://github.com/ws-io/ws.io-rs/commit/1bc8350))

### 🚀 Enhancements

- *(client)* allow using feature flags to select rustls crypto provider ([7e005f4](https://github.com/ws-io/ws.io-rs/commit/7e005f4))

### 🩹 Fixes

- *(client)* replace wsio-client feature `tls-rustls-native` with `tls-rustls-webpki` ([4fc6b8a](https://github.com/ws-io/ws.io-rs/commit/4fc6b8a))

## 0.1.0 - 2026-03-25 17:41

[compare changes](https://github.com/kiki-kanri/rs-clip-bridge/releases/tag/rs-clip-bridge-client-v0.1.0)

### ✅ Tests

- add units ([fbcdeb5](https://github.com/ws-io/ws.io-rs/commit/fbcdeb5))

### 🏡 Chore

- *(client)* add ws.io disconnect and ready logging ([6abf157](https://github.com/ws-io/ws.io-rs/commit/6abf157))

### 💅 Refactors

- update ([5ffdf36](https://github.com/ws-io/ws.io-rs/commit/5ffdf36))

### 📖 Documentation

- update readme ([a181f32](https://github.com/ws-io/ws.io-rs/commit/a181f32))
- add readme ([2e27f4d](https://github.com/ws-io/ws.io-rs/commit/2e27f4d))

### 🚀 Enhancements

- allow use env set config path ([f6f9ee2](https://github.com/ws-io/ws.io-rs/commit/f6f9ee2))
- add command to generate config template ([6cb8ecb](https://github.com/ws-io/ws.io-rs/commit/6cb8ecb))
- E2EE and refactor codes ([5b89e83](https://github.com/ws-io/ws.io-rs/commit/5b89e83))
- initial completion of functions ([d54906a](https://github.com/ws-io/ws.io-rs/commit/d54906a))

### 🩹 Fixes

- *(client)* add ring dep ([c230249](https://github.com/ws-io/ws.io-rs/commit/c230249))
- *(client)* resolve display config issues on windows platform ([7a4b684](https://github.com/ws-io/ws.io-rs/commit/7a4b684))
