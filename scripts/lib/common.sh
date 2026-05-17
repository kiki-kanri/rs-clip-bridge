#!/usr/bin/env bash

prepend_cargo_bin_to_path() {
    if [[ -d "${HOME}/.cargo/bin" ]]; then
        export PATH="${HOME}/.cargo/bin:${PATH}"
    fi
}

require_command() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "missing $1" >&2
        exit 1
    fi
}

ensure_cargo_target() {
    local target="$1"

    require_command rustup
    if rustup target list --installed | grep -Fxq "${target}"; then
        return 0
    fi

    rustup target add "${target}"
}

require_cargo_zigbuild() {
    require_command zig
    require_command cargo-zigbuild
}

encode_rustflags() {
    local sep=$'\x1f'
    local encoded=""
    local flag

    for flag in "$@"; do
        if [[ -n "${encoded}" ]]; then
            encoded+="${sep}"
        fi

        encoded+="${flag}"
    done

    printf '%s' "${encoded}"
}

exec_with_encoded_rustflags() {
    if (($# == 0)); then
        echo 'missing command' >&2
        exit 1
    fi

    if ((${#rustflags[@]} == 0)); then
        exec "$@"
    fi

    exec env CARGO_ENCODED_RUSTFLAGS="$(encode_rustflags "${rustflags[@]}")" "$@"
}
