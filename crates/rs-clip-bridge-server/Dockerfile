# Build stage
FROM rust:slim-trixie AS build-stage

## Set args, envs and workdir
ARG TARGETPLATFORM
WORKDIR /app

## Upgrade and install packages
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends gcc libssl-dev make mold pkg-config

## Pre-install toolchain using rust-toolchain.toml
COPY ./rust-toolchain.toml ./
RUN rustup show

## Copy Cargo config and manifest for dependency caching
COPY ./.cargo ./.cargo
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./crates/rs-clip-bridge-server/Cargo.toml ./crates/rs-clip-bridge-server/

## Prebuild a dummy project to populate compiled dependency cache.
## Fetch-only is not enough: the later release clippy/build steps can reuse release-profile deps from this layer/cache.
RUN --mount=type=cache,id=rs-clip-bridge-server-cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=rs-clip-bridge-server-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=rs-clip-bridge-server-target-${TARGETPLATFORM},target=/cargo-target \
    mkdir -p ./crates/rs-clip-bridge-server/src && \
    echo 'fn main() {}' >./crates/rs-clip-bridge-server/src/main.rs && \
    CARGO_TARGET_DIR=/cargo-target cargo b -r -p rs-clip-bridge-server --all-features --all-targets --locked && \
    rm -rf ./crates/rs-clip-bridge-server/src

## Copy actual source code and build with lint + format checks
COPY ./rustfmt.toml ./
COPY ./crates/rs-clip-bridge-server ./crates/rs-clip-bridge-server
RUN --mount=type=cache,id=rs-clip-bridge-server-cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=rs-clip-bridge-server-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=rs-clip-bridge-server-target-${TARGETPLATFORM},target=/cargo-target \
    touch ./crates/rs-clip-bridge-server/src/main.rs && \
    cargo format && \
    CARGO_TARGET_DIR=/cargo-target cargo clippy -r --all-features --all-targets --locked -- -D warnings && \
    CARGO_TARGET_DIR=/cargo-target cargo b -r -p rs-clip-bridge-server --all-features --locked && \
    cp -p /cargo-target/release/rs-clip-bridge-server ./rs-clip-bridge-server

# Runtime stage
FROM debian:trixie-slim

## Labels
LABEL org.opencontainers.image.title="rs-clip-bridge-server"
LABEL org.opencontainers.image.description="A blazingly fast, cross-platform clipboard synchronizer using WebSockets with end-to-end ChaCha20-Poly1305 encryption."
LABEL org.opencontainers.image.source="https://github.com/kiki-kanri/rs-clip-bridge"
LABEL org.opencontainers.image.licenses="MIT"

## Set envs and workdir
ENV RS_CLIP_SERVER_HOST='0.0.0.0' \
    RS_CLIP_SERVER_PORT=8000

WORKDIR /app

## Setups
RUN \
    ### Upgrade and install packages
    apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends tini && \
    ### Cleanup
    apt-get autoremove -y --purge && \
    apt-get clean && \
    rm -rf /var/cache/apt/* /var/lib/apt/lists/* && \
    ### Add user
    useradd -mr -g nogroup -s /usr/sbin/nologin -u 10001 user

## Copy files and libraries
COPY --from=build-stage /app/rs-clip-bridge-server ./

## Copy and set the entrypoint script
COPY --chmod=700 ./crates/rs-clip-bridge-server/docker-entrypoint.sh ./
RUN chown -R 10001:nogroup /app
USER 10001
ENTRYPOINT ["tini", "--"]
CMD ["./docker-entrypoint.sh"]
