# Build stage
FROM rust:slim-trixie AS build-stage

## Set args, envs and workdir
ENV RUSTFLAGS='-C link-arg=-fuse-ld=mold -C relro-level=full'
WORKDIR /app

## Upgrade and install packages
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends gcc libssl-dev make mold pkg-config

## Add rustup components
RUN rustup component add clippy && \
    rustup component add --toolchain nightly rustfmt

## Copy Cargo config and manifest for dependency caching
COPY ./.cargo ./.cargo
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./crates/rs-clip-bridge-server/Cargo.toml ./crates/rs-clip-bridge-server/

## Prebuild dummy project to populate Cargo dependency cache
RUN mkdir -p ./crates/rs-clip-bridge-server/src && \
    echo 'fn main() {}' >./crates/rs-clip-bridge-server/src/main.rs && \
    cargo b -r -p rs-clip-bridge-server && \
    rm -rf ./crates/rs-clip-bridge-server/src

## Copy actual source code and build with lint + format checks
COPY ./rustfmt.toml ./
COPY ./crates/rs-clip-bridge-server ./crates/rs-clip-bridge-server
RUN touch ./crates/rs-clip-bridge-server/src/main.rs && \
    cargo +nightly fmt --all -- --check && \
    cargo clippy -r --all-features --all-targets -- -D warnings && \
    cargo b -r -p rs-clip-bridge-server --all-features

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
COPY --from=build-stage /app/target/release/rs-clip-bridge-server ./

## Copy and set the entrypoint script
COPY --chmod=700 ./crates/rs-clip-bridge-server/docker-entrypoint.sh ./
RUN chown -R 10001:nogroup /app
USER 10001
ENTRYPOINT ["tini", "--"]
CMD ["./docker-entrypoint.sh"]
