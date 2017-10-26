#!/bin/bash
set -euo pipefail
export CARGO_TARGET_DIR=$(mktemp -d)

(
    if [[ $# -gt 0 ]]; then
        yum install -y "$@"
    fi
    . $HOME/.cargo/env
    cargo build ${CARGO_FLAGS:-} --release
) 1>&2
cd $CARGO_TARGET_DIR/release
(
    strip liblambda.so
    zip lambda.zip liblambda.so
) 1>&2
exec cat lambda.zip
