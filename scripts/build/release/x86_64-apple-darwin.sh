#!/usr/bin/env bash

set -euo pipefail

sep=$'\x1f'
flags=(
    # Optional CPU baseline tuning for deployment fleets with known x86-64
    # support. Keep disabled for generic release binaries; x86-64-v3, for
    # example, requires AVX/AVX2-class machines and excludes older Intel Macs.
    # -C target-cpu=x86-64-v2
    # -C target-cpu=x86-64-v3

    # Optional CPU extensions. Keep disabled for generic release binaries; use
    # only when all target machines are known to support the selected feature.
    # -C target-feature=+aes
    # -C target-feature=+avx2
    # -C target-feature=+sse4.2
)

if ((${#flags[@]} == 0)); then
    exec cargo b -r --target x86_64-apple-darwin "$@"
fi

encoded=""
for flag in "${flags[@]}"; do
    if [[ -n "${encoded}" ]]; then
        encoded+="$sep"
    fi

    encoded+="$flag"
done

exec env CARGO_ENCODED_RUSTFLAGS="${encoded}" \
    cargo b -r --target x86_64-apple-darwin "$@"
