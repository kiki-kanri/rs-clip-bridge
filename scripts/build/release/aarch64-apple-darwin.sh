#!/usr/bin/env bash

set -euo pipefail

sep=$'\x1f'
flags=(
    # Optional CPU tuning for deployment fleets with a known Apple Silicon
    # baseline. Keep disabled for generic release binaries because it can emit
    # instructions that are unavailable on older Apple Silicon machines.
    # -C target-cpu=apple-m1
    # -C target-cpu=apple-m2

    # Optional ARMv8 extensions. Keep disabled for generic release binaries; use
    # only when all target machines are known to support the selected feature.
    # -C target-feature=+crc
    # -C target-feature=+crypto
    # -C target-feature=+lse
)

if ((${#flags[@]} == 0)); then
    exec cargo b -r --target aarch64-apple-darwin "$@"
fi

encoded=""
for flag in "${flags[@]}"; do
    if [[ -n "${encoded}" ]]; then
        encoded+="$sep"
    fi

    encoded+="$flag"
done

exec env CARGO_ENCODED_RUSTFLAGS="${encoded}" \
    cargo b -r --target aarch64-apple-darwin "$@"
