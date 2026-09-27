# Build stage
FROM rust:slim-trixie AS build-stage

## Upgrade system packages
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends gcc libssl-dev make mold pkg-config

## Configure working directory
WORKDIR /app

## Pre-install toolchain using rust-toolchain.toml
COPY ./rust-toolchain.toml ./
RUN rustup show

## Copy Cargo configuration and dependency manifests for caching
COPY ./.cargo ./.cargo
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./crates/rs-clip-bridge-client/Cargo.toml ./crates/rs-clip-bridge-client/
COPY ./crates/rs-clip-bridge-server/Cargo.toml ./crates/rs-clip-bridge-server/

## Set args
ARG TARGETPLATFORM

## Prebuild a dummy project to populate the compiled dependency cache
## Release-mode dependencies are reused by the subsequent lint and build steps
RUN --mount=type=cache,id=rs-clip-bridge-server-cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=rs-clip-bridge-server-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=rs-clip-bridge-server-target-${TARGETPLATFORM},target=/cargo-target \
    mkdir -p ./crates/rs-clip-bridge-client/src ./crates/rs-clip-bridge-server/src && \
    echo 'fn main() {}' >./crates/rs-clip-bridge-client/src/main.rs && \
    echo 'fn main() {}' >./crates/rs-clip-bridge-server/src/main.rs && \
    CARGO_TARGET_DIR=/cargo-target cargo b -r -p rs-clip-bridge-server --all-features --all-targets --locked && \
    rm -rf ./crates/rs-clip-bridge-server/src

## Copy application sources and build configuration
COPY ./rustfmt.toml ./
COPY ./crates/rs-clip-bridge-server ./crates/rs-clip-bridge-server

## Validate and build the application
RUN --mount=type=cache,id=rs-clip-bridge-server-cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=rs-clip-bridge-server-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=rs-clip-bridge-server-target-${TARGETPLATFORM},target=/cargo-target \
    touch ./crates/rs-clip-bridge-server/src/main.rs && \
    cargo fmt -p rs-clip-bridge-server -- --check && \
    CARGO_TARGET_DIR=/cargo-target cargo clippy -r -p rs-clip-bridge-server --all-features --all-targets --locked -- -D warnings && \
    CARGO_TARGET_DIR=/cargo-target cargo b -r -p rs-clip-bridge-server --all-features --locked && \
    cp -p /cargo-target/release/rs-clip-bridge-server ./rs-clip-bridge-server

# Runtime stage
FROM debian:trixie-slim

## Labels
LABEL org.opencontainers.image.title="rs-clip-bridge-server"
LABEL org.opencontainers.image.description="A blazingly fast, cross-platform clipboard synchronizer using WebSockets with end-to-end ChaCha20-Poly1305 encryption."
LABEL org.opencontainers.image.source="https://github.com/kiki-kanri/rs-clip-bridge"
LABEL org.opencontainers.image.licenses="MIT"

## Configure working directory
WORKDIR /app

## Install runtime packages and configure the runtime user
RUN \
    ### Upgrade system packages and install runtime dependencies
    apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends tini && \
    ### Clean package manager metadata
    apt-get autoremove -y --purge && \
    apt-get clean && \
    rm -rf /var/cache/apt/* /var/lib/apt/lists/* && \
    ### Create the runtime user and set application ownership
    useradd -mr -g nogroup -s /usr/sbin/nologin -u 10001 user && \
    chown 10001:nogroup /app -R

## Copy and configure the entrypoint
COPY --chmod=700 --chown=10001:nogroup ./crates/rs-clip-bridge-server/docker-entrypoint.sh ./
USER 10001
ENTRYPOINT ["tini", "--"]
CMD ["./docker-entrypoint.sh"]

## Configure remaining runtime defaults
ENV RS_CLIP_SERVER_HOST='0.0.0.0' \
    RS_CLIP_SERVER_PORT=8000

## Copy application output and runtime configuration
COPY --chown=10001:nogroup --from=build-stage /app/rs-clip-bridge-server ./
