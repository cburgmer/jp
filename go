#!/usr/bin/env bash
set -euo pipefail

readonly SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

(
  cd "$SCRIPT_DIR"
  cargo build
  PATH="$SCRIPT_DIR/target/debug:$PATH" ./tests.bats
)
