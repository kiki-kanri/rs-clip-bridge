#!/bin/bash

set -e

SCRIPT_DIR="$(dirname "$(realpath "${BASH_SOURCE[0]}")")"
cd "${SCRIPT_DIR}"

PLATFORMS='linux/amd64,linux/arm64'

docker pull rust:slim-trixie &
docker pull debian:trixie-slim &
wait

docker buildx build \
    -f ./crates/rs-clip-bridge-server/Dockerfile \
    -t 'kikikanri/rs-clip-bridge-server:latest' \
    --builder multi-platform \
    --no-cache \
    --platform "${PLATFORMS}" \
    --push \
    .
