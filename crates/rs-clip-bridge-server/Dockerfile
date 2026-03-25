# Build stage
FROM rust:slim-trixie AS build-stage

## Set args, envs and workdir
ENV RUSTFLAGS='-C link-arg=-fuse-ld=mold -C relro-level=full'
WORKDIR /app

## Upgrade and install packages
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends gcc make mold

## Add rustup components
RUN rustup component add clippy && \
    rustup component add --toolchain nightly rustfmt

## Copy Cargo config and manifest for dependency caching
COPY ./.cargo ./.cargo
COPY ./Cargo.lock ./Cargo.toml ./

## Prebuild dummy project to populate Cargo dependency cache
RUN mkdir ./src && \
    echo 'fn main() {}' >./src/main.rs && \
    cargo b -r && \
    rm -rf ./src

## Copy actual source code and build with lint + format checks
COPY ./rustfmt.toml ./
COPY ./src ./src
RUN touch ./src/main.rs && \
    cargo +nightly fmt --all -- --check && \
    cargo clippy -r --all-features --all-targets -- -D warnings && \
    cargo b -r --all-features

# Runtime stage
FROM debian:trixie-slim

## Set envs and workdir
ENV SERVER_HOST='0.0.0.0' \
    SERVER_PORT=8000

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
COPY --chmod=700 ./docker-entrypoint.sh ./
RUN chown -R 10001:nogroup /app
USER 10001
ENTRYPOINT ["tini", "--"]
CMD ["./docker-entrypoint.sh"]
