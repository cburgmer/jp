#!/usr/bin/env bash
set -euo pipefail

readonly SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

if ! which bats > /dev/null; then
    echo >&2 "Please install https://github.com/bats-core/bats-core to run the tests"
    exit 1
fi

if ! which cargo > /dev/null; then
    echo >&2 "Please install Rust and Cargo to run the tests"
    exit 1
fi

(
  cd "$SCRIPT_DIR"
  cargo build
  PATH="$SCRIPT_DIR/target/debug:$PATH" bats -t ./tests.bats
)
