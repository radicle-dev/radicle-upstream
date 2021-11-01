#!/bin/bash

# Copyright © 2021 The Radicle Upstream Contributors
#
# This file is part of radicle-upstream, distributed under the GPLv3
# with Radicle Linking Exception. For full terms see the included
# LICENSE file.

source ci/env.sh

log-group-start "Installing yarn dependencies"
yarn install --immutable
yarn dedupe --check
log-group-end

log-group-start "Linking cache"
declare -r rust_target_cache="$CACHE_FOLDER/proxy-target"
mkdir -p "$rust_target_cache"
ln -s "${rust_target_cache}" ./target

declare -r cargo_deny_cache="$CACHE_FOLDER/cargo-deny"
mkdir -p "$cargo_deny_cache"
mkdir -p ~/.cargo
ln -sf "$cargo_deny_cache" ~/.cargo/advisory-db

if [[ "${BUILDKITE_AGENT_META_DATA_PLATFORM:-}" != "macos" && "${RUNNER_OS:-}" != "macOS" ]]; then
  free_cache_space_kb=$(df --output=avail /cache | sed -n 2p)
  min_free_cache_kb=$(( 2 * 1024 * 1024 )) # 2GiB is 25%
  echo "$(( free_cache_space_kb / 1024 )) MiB free space on /cache"
  if [[ $free_cache_space_kb -le $min_free_cache_kb ]]; then
    echo "Not enough free space on /cache. Deleting ${rust_target_cache}"
    du -sh /cache/*
    rm -rf "${rust_target_cache}"
    mkdir -p "${rust_target_cache}"
  fi
fi
log-group-end

log-group-start "Updating submodules"
./scripts/test-setup.sh
log-group-end

log-group-start "Set custom git config"
cp .buildkite/.gitconfig "$HOME/"
cat "$HOME/.gitconfig"
log-group-end

log-group-start "License compliance"
time ./scripts/license-header.ts check
time cargo deny check
log-group-end

log-group-start "Run proxy fmt"
time cargo fmt --all -- --check
log-group-end

log-group-start "Run proxy lints"
time cargo clippy --all --all-targets -- --deny warnings
time cargo clippy --all --all-targets --all-features -- --deny warnings
log-group-end

log-group-start "Run proxy docs"
(
  export RUSTDOCFLAGS="-D broken-intra-doc-links"
  time cargo doc --workspace --no-deps --all-features --document-private-items
)
log-group-end

log-group-start "Run app eslint checks"
time yarn lint
log-group-end

log-group-start "Run app prettier checks"
time yarn prettier:check
log-group-end

log-group-start "Check TypeScript"
time yarn typescript:check
log-group-end

log-group-start "Run proxy tests"
(
  export RUST_TEST_TIME_UNIT=2000,4000
  export RUST_TEST_TIME_INTEGRATION=2000,8000
  cargo test --all --all-features --all-targets -- -Z unstable-options --report-time
)
log-group-end

log-group-start "Bundle electron main files"
time yarn run webpack --config-name main
log-group-end

log-group-start "Starting proxy daemon and runing app tests"
# We modify the output of the tests to add log groups to the cypress
# tests.
time FORCE_COLOR=1 ELECTRON_ENABLE_LOGGING=1 yarn test |
  sed "
    s/^\\s*Running:/$(log-group-end)\n$(log-group-start)Running:/
    s/^.*Run Finished.*/$(log-group-end)\n$(log-group-start)Run Finished/
  "
log-group-end
