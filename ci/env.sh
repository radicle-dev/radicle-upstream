#!/bin/bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

set -Eeou pipefail

if [[ "${GITHUB_ACTIONS:-}" = "true" ]]; then
  export CACHE_FOLDER="$HOME/cache"

  function log-group-start () {
    echo "::group::${1:-}"
  }

  function log-group-end () {
    echo "::endgroup::"
  }
else
  echo "Unknown CI platform"
  exit 1
fi

# Remove cargo build artifacts that are large and become stale on every
# build.
function clean-cargo-build-artifacts () {
  if [[ ${RUNNER_OS} == "macOS" ]]; then
    echo "skipping build artifacts clean-up on macOS"
  else
    echo "clean up cargo build artifacts"
    find target/*/deps -type f -perm -a=x -not -name "*.so" -exec rm "{}" ";"
    rm target/*/deps/libupstream*
  fi
}

export YARN_CACHE_FOLDER="$CACHE_FOLDER/yarn"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"

export TIMEFORMAT='elapsed time: %R (user: %U, system: %S)'
