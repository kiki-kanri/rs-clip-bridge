#!/usr/bin/env bash

set -euo pipefail

sep=$'\x1f'
flags=(
    # Optional faster ELF linker. Keep disabled by default because GitHub runners
    # and many developer machines do not have mold installed.
    # -C link-arg=-fuse-ld=mold

    # Optional CPU baseline tuning for deployment fleets with known x86-64
    # support. Keep disabled for generic release binaries; x86-64-v3, for
    # example, requires AVX/AVX2-class machines and excludes older CPUs.
    # -C target-cpu=x86-64-v2
    # -C target-cpu=x86-64-v3

    # Optional CPU extensions. Keep disabled for generic release binaries; use
    # only when all target machines are known to support the selected feature.
    # -C target-feature=+aes
    # -C target-feature=+avx2
    # -C target-feature=+sse4.2

    # Optional size/link optimization for ELF linkers that support identical code
    # folding. Keep disabled by default because --icf=all can merge functions with
    # identical machine code and therefore change function pointer identity.
    # -C link-arg=-Wl,--icf=all
)

if ((${#flags[@]} == 0)); then
    exec cargo b -r --target x86_64-unknown-linux-gnu "$@"
fi

encoded=""
for flag in "${flags[@]}"; do
    if [[ -n "${encoded}" ]]; then
        encoded+="$sep"
    fi

    encoded+="$flag"
done

exec env CARGO_ENCODED_RUSTFLAGS="${encoded}" \
    cargo b -r --target x86_64-unknown-linux-gnu "$@"
